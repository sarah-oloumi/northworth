use crate::domain::account::AccountProfile;
use crate::domain::reports::{
    build_calendar_report, build_cash_flow_report, build_net_worth_snapshot,
    build_spending_analysis_report, build_transaction_summary_card, CalendarReport, CashFlowReport,
    NetWorthSnapshot, ReportDateRange, SpendingAnalysisReport, TransactionSummaryCard,
};
use crate::domain::transaction::TransactionRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionReportInput {
    pub transactions: Vec<TransactionRecord>,
    pub accounts: Vec<AccountProfile>,
    pub date_range: ReportDateRange,
}

#[tauri::command]
pub fn build_cash_flow(input: TransactionReportInput) -> CashFlowReport {
    build_cash_flow_report(&input.transactions, &input.accounts, input.date_range)
}

#[tauri::command]
pub fn build_spending_analysis(input: TransactionReportInput) -> SpendingAnalysisReport {
    build_spending_analysis_report(&input.transactions, &input.accounts, input.date_range)
}

#[tauri::command]
pub fn build_transaction_summary(input: TransactionReportInput) -> TransactionSummaryCard {
    build_transaction_summary_card(&input.transactions, &input.accounts, input.date_range)
}

#[tauri::command]
pub fn build_calendar(input: TransactionReportInput) -> CalendarReport {
    build_calendar_report(&input.transactions, &input.accounts, input.date_range)
}

#[tauri::command]
pub fn build_net_worth(accounts: Vec<AccountProfile>) -> NetWorthSnapshot {
    build_net_worth_snapshot(&accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::account::{AccountId, AccountKind, AccountStatus, BudgetTreatment};
    use crate::domain::money::MoneyAmount;
    use crate::domain::reports::{CalendarDay, CashFlowMonth, ReportMonth};
    use crate::domain::transaction::{ImportReviewStatus, TransactionCategory, TransactionDate};

    #[test]
    fn builds_cash_flow_through_command() {
        let chequing = account_id("chequing");
        let input = TransactionReportInput {
            transactions: vec![
                transaction(2026, 1, 1, 100_000, Some(chequing.clone())),
                transaction(2026, 1, 2, -25_000, Some(chequing.clone())),
            ],
            accounts: vec![account(chequing, BudgetTreatment::OnBudget, 0)],
            date_range: range(),
        };

        let report = build_cash_flow(input);

        assert_eq!(
            report.months,
            vec![CashFlowMonth {
                month: ReportMonth {
                    year: 2026,
                    month: 1,
                },
                income: MoneyAmount::cad_cents(100_000),
                expenses: MoneyAmount::cad_cents(25_000),
                net_cash_flow: MoneyAmount::cad_cents(75_000),
            }]
        );
    }

    #[test]
    fn builds_spending_analysis_through_command() {
        let chequing = account_id("chequing");
        let mut transaction = transaction(2026, 1, 1, -25_000, Some(chequing.clone()));
        transaction.category = Some(TransactionCategory::Groceries);
        let input = TransactionReportInput {
            transactions: vec![transaction],
            accounts: vec![account(chequing, BudgetTreatment::OnBudget, 0)],
            date_range: range(),
        };

        let report = build_spending_analysis(input);

        assert_eq!(report.total_spending, MoneyAmount::cad_cents(25_000));
        assert_eq!(
            report.categories[0].category,
            TransactionCategory::Groceries
        );
    }

    #[test]
    fn builds_summary_card_through_command() {
        let chequing = account_id("chequing");
        let input = TransactionReportInput {
            transactions: vec![
                transaction(2026, 1, 1, 100_000, Some(chequing.clone())),
                transaction(2026, 1, 2, -25_000, Some(chequing.clone())),
            ],
            accounts: vec![account(chequing, BudgetTreatment::OnBudget, 0)],
            date_range: range(),
        };

        let report = build_transaction_summary(input);

        assert_eq!(report.transaction_count, 2);
        assert_eq!(report.net_total, MoneyAmount::cad_cents(75_000));
    }

    #[test]
    fn builds_calendar_through_command() {
        let chequing = account_id("chequing");
        let input = TransactionReportInput {
            transactions: vec![
                transaction(2026, 1, 1, 100_000, Some(chequing.clone())),
                transaction(2026, 1, 1, -25_000, Some(chequing.clone())),
            ],
            accounts: vec![account(chequing, BudgetTreatment::OnBudget, 0)],
            date_range: range(),
        };

        let report = build_calendar(input);

        assert_eq!(
            report.days,
            vec![CalendarDay {
                date: date(2026, 1, 1),
                income: MoneyAmount::cad_cents(100_000),
                expenses: MoneyAmount::cad_cents(25_000),
                net: MoneyAmount::cad_cents(75_000),
            }]
        );
    }

    #[test]
    fn builds_net_worth_through_command() {
        let snapshot = build_net_worth(vec![
            account(account_id("chequing"), BudgetTreatment::OnBudget, 100_000),
            account(
                account_id("mortgage"),
                BudgetTreatment::OffBudgetTracking,
                -400_000,
            ),
        ]);

        assert_eq!(snapshot.assets, MoneyAmount::cad_cents(100_000));
        assert_eq!(snapshot.debts, MoneyAmount::cad_cents(400_000));
        assert_eq!(snapshot.net_worth, MoneyAmount::cad_cents(-300_000));
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
            account_name: None,
            category: None,
            import_status: ImportReviewStatus::PendingReview,
        }
    }

    fn account(
        id: AccountId,
        budget_treatment: BudgetTreatment,
        balance_cents: i64,
    ) -> AccountProfile {
        AccountProfile {
            id,
            name: "Demo account".to_string(),
            kind: AccountKind::Chequing,
            budget_treatment,
            status: AccountStatus::Open,
            current_balance: MoneyAmount::cad_cents(balance_cents),
        }
    }

    fn range() -> ReportDateRange {
        ReportDateRange {
            start: date(2026, 1, 1),
            end: date(2026, 12, 31),
        }
    }

    fn date(year: u16, month: u8, day: u8) -> TransactionDate {
        TransactionDate { year, month, day }
    }

    fn account_id(value: &str) -> AccountId {
        AccountId::new(value).expect("valid account id")
    }
}
