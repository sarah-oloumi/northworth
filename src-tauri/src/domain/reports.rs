use super::account::{AccountId, AccountProfile, BudgetTreatment};
use super::budget::{summarize_for_budget_pies_with_accounts, BudgetSlice};
use super::money::MoneyAmount;
use super::transaction::{TransactionDate, TransactionRecord};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportDateRange {
    pub start: TransactionDate,
    pub end: TransactionDate,
}

impl ReportDateRange {
    pub fn includes(&self, date: TransactionDate) -> bool {
        self.start <= date && date <= self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CashFlowReport {
    pub months: Vec<CashFlowMonth>,
    pub income: MoneyAmount,
    pub expenses: MoneyAmount,
    pub net_cash_flow: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CashFlowMonth {
    pub month: ReportMonth,
    pub income: MoneyAmount,
    pub expenses: MoneyAmount,
    pub net_cash_flow: MoneyAmount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ReportMonth {
    pub year: u16,
    pub month: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendingAnalysisReport {
    pub categories: Vec<BudgetSlice>,
    pub total_spending: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionSummaryCard {
    pub transaction_count: usize,
    pub total_income: MoneyAmount,
    pub total_expenses: MoneyAmount,
    pub net_total: MoneyAmount,
    pub average_monthly_income: MoneyAmount,
    pub average_monthly_expenses: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarReport {
    pub days: Vec<CalendarDay>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarDay {
    pub date: TransactionDate,
    pub income: MoneyAmount,
    pub expenses: MoneyAmount,
    pub net: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetWorthSnapshot {
    pub accounts: Vec<NetWorthAccount>,
    pub assets: MoneyAmount,
    pub debts: MoneyAmount,
    pub net_worth: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetWorthAccount {
    pub account_id: AccountId,
    pub name: String,
    pub balance: MoneyAmount,
}

pub fn build_cash_flow_report(
    transactions: &[TransactionRecord],
    accounts: &[AccountProfile],
    date_range: ReportDateRange,
) -> CashFlowReport {
    let mut months = BTreeMap::<ReportMonth, CashFlowAccumulator>::new();

    for transaction in filter_budgeted_transactions(transactions, accounts) {
        if !date_range.includes(transaction.date) {
            continue;
        }

        let accumulator = months
            .entry(ReportMonth::from(transaction.date))
            .or_default();

        if transaction.amount.is_inflow() {
            accumulator.income_cents += transaction.amount.cents;
        } else if transaction.amount.is_outflow() {
            accumulator.expense_cents += transaction.amount.cents.abs();
        }
    }

    let months = months
        .into_iter()
        .map(|(month, accumulator)| CashFlowMonth {
            month,
            income: MoneyAmount::cad_cents(accumulator.income_cents),
            expenses: MoneyAmount::cad_cents(accumulator.expense_cents),
            net_cash_flow: MoneyAmount::cad_cents(
                accumulator.income_cents - accumulator.expense_cents,
            ),
        })
        .collect::<Vec<_>>();

    let income_cents = months.iter().map(|month| month.income.cents).sum::<i64>();
    let expense_cents = months.iter().map(|month| month.expenses.cents).sum::<i64>();

    CashFlowReport {
        months,
        income: MoneyAmount::cad_cents(income_cents),
        expenses: MoneyAmount::cad_cents(expense_cents),
        net_cash_flow: MoneyAmount::cad_cents(income_cents - expense_cents),
    }
}

pub fn build_spending_analysis_report(
    transactions: &[TransactionRecord],
    accounts: &[AccountProfile],
    date_range: ReportDateRange,
) -> SpendingAnalysisReport {
    let transactions = transactions
        .iter()
        .filter(|transaction| date_range.includes(transaction.date))
        .cloned()
        .collect::<Vec<_>>();
    let summary = summarize_for_budget_pies_with_accounts(&transactions, accounts);
    let total_spending = summary.total_outflow_cents();

    SpendingAnalysisReport {
        categories: summary.outflow,
        total_spending: MoneyAmount::cad_cents(total_spending),
    }
}

pub fn build_transaction_summary_card(
    transactions: &[TransactionRecord],
    accounts: &[AccountProfile],
    date_range: ReportDateRange,
) -> TransactionSummaryCard {
    let mut month_count = 0;
    let mut seen_months = HashSet::new();
    let mut transaction_count = 0;
    let mut income_cents = 0;
    let mut expense_cents = 0;

    for transaction in filter_budgeted_transactions(transactions, accounts) {
        if !date_range.includes(transaction.date) {
            continue;
        }

        transaction_count += 1;
        seen_months.insert(ReportMonth::from(transaction.date));

        if transaction.amount.is_inflow() {
            income_cents += transaction.amount.cents;
        } else if transaction.amount.is_outflow() {
            expense_cents += transaction.amount.cents.abs();
        }
    }

    if !seen_months.is_empty() {
        month_count = seen_months.len() as i64;
    }

    TransactionSummaryCard {
        transaction_count,
        total_income: MoneyAmount::cad_cents(income_cents),
        total_expenses: MoneyAmount::cad_cents(expense_cents),
        net_total: MoneyAmount::cad_cents(income_cents - expense_cents),
        average_monthly_income: MoneyAmount::cad_cents(divide_cents(income_cents, month_count)),
        average_monthly_expenses: MoneyAmount::cad_cents(divide_cents(expense_cents, month_count)),
    }
}

pub fn build_calendar_report(
    transactions: &[TransactionRecord],
    accounts: &[AccountProfile],
    date_range: ReportDateRange,
) -> CalendarReport {
    let mut days = BTreeMap::<TransactionDate, CalendarAccumulator>::new();

    for transaction in filter_budgeted_transactions(transactions, accounts) {
        if !date_range.includes(transaction.date) {
            continue;
        }

        let accumulator = days.entry(transaction.date).or_default();

        if transaction.amount.is_inflow() {
            accumulator.income_cents += transaction.amount.cents;
        } else if transaction.amount.is_outflow() {
            accumulator.expense_cents += transaction.amount.cents.abs();
        }
    }

    CalendarReport {
        days: days
            .into_iter()
            .map(|(date, accumulator)| CalendarDay {
                date,
                income: MoneyAmount::cad_cents(accumulator.income_cents),
                expenses: MoneyAmount::cad_cents(accumulator.expense_cents),
                net: MoneyAmount::cad_cents(accumulator.income_cents - accumulator.expense_cents),
            })
            .collect(),
    }
}

pub fn build_net_worth_snapshot(accounts: &[AccountProfile]) -> NetWorthSnapshot {
    let mut assets_cents = 0;
    let mut debts_cents = 0;

    let accounts = accounts
        .iter()
        .map(|account| {
            if account.current_balance.cents >= 0 {
                assets_cents += account.current_balance.cents;
            } else {
                debts_cents += account.current_balance.cents.abs();
            }

            NetWorthAccount {
                account_id: account.id.clone(),
                name: account.name.clone(),
                balance: account.current_balance.clone(),
            }
        })
        .collect::<Vec<_>>();

    NetWorthSnapshot {
        accounts,
        assets: MoneyAmount::cad_cents(assets_cents),
        debts: MoneyAmount::cad_cents(debts_cents),
        net_worth: MoneyAmount::cad_cents(assets_cents - debts_cents),
    }
}

fn filter_budgeted_transactions<'a>(
    transactions: &'a [TransactionRecord],
    accounts: &'a [AccountProfile],
) -> impl Iterator<Item = &'a TransactionRecord> {
    let on_budget_accounts = accounts
        .iter()
        .filter(|account| account.budget_treatment == BudgetTreatment::OnBudget)
        .map(|account| account.id.clone())
        .collect::<HashSet<_>>();

    transactions.iter().filter(move |transaction| {
        transaction
            .account_id
            .as_ref()
            .map(|account_id| on_budget_accounts.contains(account_id))
            .unwrap_or(true)
    })
}

impl From<TransactionDate> for ReportMonth {
    fn from(date: TransactionDate) -> Self {
        Self {
            year: date.year,
            month: date.month,
        }
    }
}

fn divide_cents(cents: i64, divisor: i64) -> i64 {
    if divisor == 0 {
        0
    } else {
        cents / divisor
    }
}

#[derive(Debug, Default)]
struct CashFlowAccumulator {
    income_cents: i64,
    expense_cents: i64,
}

#[derive(Debug, Default)]
struct CalendarAccumulator {
    income_cents: i64,
    expense_cents: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::account::{AccountKind, AccountStatus};
    use crate::domain::transaction::{ImportReviewStatus, TransactionCategory};

    #[test]
    fn builds_cash_flow_from_on_budget_accounts_by_month() {
        let chequing = account_id("chequing");
        let investment = account_id("investment");
        let accounts = vec![
            account(
                chequing.clone(),
                "Chequing",
                BudgetTreatment::OnBudget,
                100_000,
            ),
            account(
                investment.clone(),
                "Investment",
                BudgetTreatment::OffBudgetTracking,
                500_000,
            ),
        ];
        let transactions = vec![
            transaction(2026, 1, 1, 500_000, Some(chequing.clone())),
            transaction(2026, 1, 5, -100_000, Some(chequing.clone())),
            transaction(2026, 2, 1, -25_000, Some(chequing)),
            transaction(2026, 2, 2, 1_000_000, Some(investment)),
        ];

        let report = build_cash_flow_report(&transactions, &accounts, range());

        assert_eq!(report.income, MoneyAmount::cad_cents(500_000));
        assert_eq!(report.expenses, MoneyAmount::cad_cents(125_000));
        assert_eq!(report.net_cash_flow, MoneyAmount::cad_cents(375_000));
        assert_eq!(
            report.months,
            vec![
                CashFlowMonth {
                    month: ReportMonth {
                        year: 2026,
                        month: 1,
                    },
                    income: MoneyAmount::cad_cents(500_000),
                    expenses: MoneyAmount::cad_cents(100_000),
                    net_cash_flow: MoneyAmount::cad_cents(400_000),
                },
                CashFlowMonth {
                    month: ReportMonth {
                        year: 2026,
                        month: 2,
                    },
                    income: MoneyAmount::cad_cents(0),
                    expenses: MoneyAmount::cad_cents(25_000),
                    net_cash_flow: MoneyAmount::cad_cents(-25_000),
                },
            ]
        );
    }

    #[test]
    fn builds_spending_analysis_from_categories() {
        let chequing = account_id("chequing");
        let accounts = vec![account(
            chequing.clone(),
            "Chequing",
            BudgetTreatment::OnBudget,
            100_000,
        )];
        let mut grocery = transaction(2026, 1, 1, -12_000, Some(chequing.clone()));
        grocery.category = Some(TransactionCategory::Groceries);
        let mut gas = transaction(2026, 1, 5, -5_000, Some(chequing));
        gas.category = Some(TransactionCategory::Gas);

        let report = build_spending_analysis_report(&[grocery, gas], &accounts, range());

        assert_eq!(report.total_spending, MoneyAmount::cad_cents(17_000));
        assert_eq!(report.categories.len(), 2);
    }

    #[test]
    fn builds_summary_card_with_monthly_average() {
        let chequing = account_id("chequing");
        let accounts = vec![account(
            chequing.clone(),
            "Chequing",
            BudgetTreatment::OnBudget,
            100_000,
        )];
        let transactions = vec![
            transaction(2026, 1, 1, 300_000, Some(chequing.clone())),
            transaction(2026, 1, 2, -50_000, Some(chequing.clone())),
            transaction(2026, 2, 1, 300_000, Some(chequing)),
        ];

        let report = build_transaction_summary_card(&transactions, &accounts, range());

        assert_eq!(report.transaction_count, 3);
        assert_eq!(report.total_income, MoneyAmount::cad_cents(600_000));
        assert_eq!(report.total_expenses, MoneyAmount::cad_cents(50_000));
        assert_eq!(
            report.average_monthly_income,
            MoneyAmount::cad_cents(300_000)
        );
        assert_eq!(
            report.average_monthly_expenses,
            MoneyAmount::cad_cents(25_000)
        );
    }

    #[test]
    fn builds_calendar_report_by_day() {
        let chequing = account_id("chequing");
        let accounts = vec![account(
            chequing.clone(),
            "Chequing",
            BudgetTreatment::OnBudget,
            100_000,
        )];
        let transactions = vec![
            transaction(2026, 1, 1, 300_000, Some(chequing.clone())),
            transaction(2026, 1, 1, -50_000, Some(chequing)),
        ];

        let report = build_calendar_report(&transactions, &accounts, range());

        assert_eq!(
            report.days,
            vec![CalendarDay {
                date: date(2026, 1, 1),
                income: MoneyAmount::cad_cents(300_000),
                expenses: MoneyAmount::cad_cents(50_000),
                net: MoneyAmount::cad_cents(250_000),
            }]
        );
    }

    #[test]
    fn builds_net_worth_snapshot_from_all_accounts() {
        let accounts = vec![
            account(
                account_id("chequing"),
                "Chequing",
                BudgetTreatment::OnBudget,
                100_000,
            ),
            account(
                account_id("mortgage"),
                "Mortgage",
                BudgetTreatment::OffBudgetTracking,
                -400_000,
            ),
            account(
                account_id("investment"),
                "Investment",
                BudgetTreatment::OffBudgetTracking,
                900_000,
            ),
        ];

        let snapshot = build_net_worth_snapshot(&accounts);

        assert_eq!(snapshot.assets, MoneyAmount::cad_cents(1_000_000));
        assert_eq!(snapshot.debts, MoneyAmount::cad_cents(400_000));
        assert_eq!(snapshot.net_worth, MoneyAmount::cad_cents(600_000));
        assert_eq!(snapshot.accounts.len(), 3);
    }

    fn account(
        id: AccountId,
        name: &str,
        budget_treatment: BudgetTreatment,
        cents: i64,
    ) -> AccountProfile {
        AccountProfile {
            id,
            name: name.to_string(),
            kind: AccountKind::Chequing,
            budget_treatment,
            status: AccountStatus::Open,
            current_balance: MoneyAmount::cad_cents(cents),
        }
    }

    fn transaction(
        year: u16,
        month: u8,
        day: u8,
        cents: i64,
        account_id: Option<AccountId>,
    ) -> TransactionRecord {
        TransactionRecord {
            source_row: 2,
            date: date(year, month, day),
            description: "Demo transaction".to_string(),
            amount: MoneyAmount::cad_cents(cents),
            account_id,
            account_name: Some("Demo account".to_string()),
            category: None,
            import_status: ImportReviewStatus::PendingReview,
        }
    }

    fn date(year: u16, month: u8, day: u8) -> TransactionDate {
        TransactionDate { year, month, day }
    }

    fn range() -> ReportDateRange {
        ReportDateRange {
            start: date(2026, 1, 1),
            end: date(2026, 12, 31),
        }
    }

    fn account_id(value: &str) -> AccountId {
        AccountId::new(value).expect("valid account id")
    }
}
