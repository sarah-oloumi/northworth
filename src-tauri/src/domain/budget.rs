use super::account::{AccountId, AccountProfile, BudgetTreatment};
use super::money::MoneyAmount;
use super::transaction::{TransactionCategory, TransactionRecord};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetSummary {
    pub inflow: Vec<BudgetSlice>,
    pub outflow: Vec<BudgetSlice>,
}

impl BudgetSummary {
    pub fn total_inflow_cents(&self) -> i64 {
        self.inflow.iter().map(|slice| slice.amount.cents).sum()
    }

    pub fn total_outflow_cents(&self) -> i64 {
        self.outflow.iter().map(|slice| slice.amount.cents).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetSlice {
    pub category: TransactionCategory,
    pub amount: MoneyAmount,
    pub transaction_count: usize,
}

pub fn summarize_for_budget_pies(transactions: &[TransactionRecord]) -> BudgetSummary {
    summarize_for_budget_pies_with_account_filter(transactions, &|_| true)
}

pub fn summarize_for_budget_pies_with_accounts(
    transactions: &[TransactionRecord],
    accounts: &[AccountProfile],
) -> BudgetSummary {
    let on_budget_accounts = accounts
        .iter()
        .filter(|account| account.budget_treatment == BudgetTreatment::OnBudget)
        .map(|account| account.id.clone())
        .collect::<std::collections::HashSet<_>>();

    summarize_for_budget_pies_with_account_filter(transactions, &|account_id| {
        account_id
            .as_ref()
            .map(|account_id| on_budget_accounts.contains(account_id))
            .unwrap_or(true)
    })
}

fn summarize_for_budget_pies_with_account_filter(
    transactions: &[TransactionRecord],
    include_account: &dyn Fn(&Option<AccountId>) -> bool,
) -> BudgetSummary {
    let mut inflow = BTreeMap::<TransactionCategory, SliceAccumulator>::new();
    let mut outflow = BTreeMap::<TransactionCategory, SliceAccumulator>::new();

    for transaction in transactions {
        if !include_account(&transaction.account_id) {
            continue;
        }

        let category = transaction
            .category
            .clone()
            .unwrap_or(TransactionCategory::Uncategorized);

        if transaction.amount.is_inflow() {
            inflow
                .entry(category)
                .or_default()
                .add(transaction.amount.cents.abs());
        } else if transaction.amount.is_outflow() {
            outflow
                .entry(category)
                .or_default()
                .add(transaction.amount.cents.abs());
        }
    }

    BudgetSummary {
        inflow: to_sorted_slices(inflow),
        outflow: to_sorted_slices(outflow),
    }
}

fn to_sorted_slices(
    accumulators: BTreeMap<TransactionCategory, SliceAccumulator>,
) -> Vec<BudgetSlice> {
    let mut slices = accumulators
        .into_iter()
        .map(|(category, accumulator)| BudgetSlice {
            category,
            amount: MoneyAmount::cad_cents(accumulator.cents),
            transaction_count: accumulator.transaction_count,
        })
        .collect::<Vec<_>>();

    slices.sort_by(|left, right| {
        right
            .amount
            .cents
            .cmp(&left.amount.cents)
            .then_with(|| right.transaction_count.cmp(&left.transaction_count))
            .then_with(|| left.category.cmp(&right.category))
    });

    slices
}

#[derive(Debug, Default)]
struct SliceAccumulator {
    cents: i64,
    transaction_count: usize,
}

impl SliceAccumulator {
    fn add(&mut self, cents: i64) {
        self.cents += cents;
        self.transaction_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::account::{AccountId, AccountKind, AccountStatus};
    use crate::domain::transaction::{ImportReviewStatus, TransactionDate};

    #[test]
    fn summarizes_transactions_into_inflow_and_outflow_slices() {
        let transactions = vec![
            transaction(TransactionCategory::Income, 500_000),
            transaction(TransactionCategory::Groceries, -12_000),
            transaction(TransactionCategory::Groceries, -8_000),
            transaction(TransactionCategory::Gas, -6_500),
            transaction(TransactionCategory::Bills, -20_000),
        ];

        let summary = summarize_for_budget_pies(&transactions);

        assert_eq!(
            summary.inflow,
            vec![BudgetSlice {
                category: TransactionCategory::Income,
                amount: MoneyAmount::cad_cents(500_000),
                transaction_count: 1,
            }]
        );
        assert_eq!(
            summary.outflow,
            vec![
                BudgetSlice {
                    category: TransactionCategory::Groceries,
                    amount: MoneyAmount::cad_cents(20_000),
                    transaction_count: 2,
                },
                BudgetSlice {
                    category: TransactionCategory::Bills,
                    amount: MoneyAmount::cad_cents(20_000),
                    transaction_count: 1,
                },
                BudgetSlice {
                    category: TransactionCategory::Gas,
                    amount: MoneyAmount::cad_cents(6_500),
                    transaction_count: 1,
                },
            ]
        );
        assert_eq!(summary.total_inflow_cents(), 500_000);
        assert_eq!(summary.total_outflow_cents(), 46_500);
    }

    #[test]
    fn excludes_off_budget_accounts_from_budget_pies() {
        let on_budget_id = account_id("chequing");
        let off_budget_id = account_id("mortgage");
        let transactions = vec![
            transaction_with_account(TransactionCategory::Income, 500_000, on_budget_id.clone()),
            transaction_with_account(TransactionCategory::Bills, -20_000, on_budget_id.clone()),
            transaction_with_account(TransactionCategory::Debt, -300_000, off_budget_id.clone()),
        ];
        let accounts = vec![
            account(
                on_budget_id,
                AccountKind::Chequing,
                BudgetTreatment::OnBudget,
                100_000,
            ),
            account(
                off_budget_id,
                AccountKind::Mortgage,
                BudgetTreatment::OffBudgetTracking,
                -500_000_00,
            ),
        ];

        let summary = summarize_for_budget_pies_with_accounts(&transactions, &accounts);

        assert_eq!(summary.total_inflow_cents(), 500_000);
        assert_eq!(summary.total_outflow_cents(), 20_000);
        assert_eq!(
            summary.outflow,
            vec![BudgetSlice {
                category: TransactionCategory::Bills,
                amount: MoneyAmount::cad_cents(20_000),
                transaction_count: 1,
            }]
        );
    }

    fn transaction(category: TransactionCategory, cents: i64) -> TransactionRecord {
        TransactionRecord {
            source_row: 2,
            date: TransactionDate {
                year: 2026,
                month: 6,
                day: 1,
            },
            description: "Demo transaction".to_string(),
            amount: MoneyAmount::cad_cents(cents),
            account_id: None,
            account_name: Some("Demo account".to_string()),
            category: Some(category),
            import_status: ImportReviewStatus::PendingReview,
        }
    }

    fn transaction_with_account(
        category: TransactionCategory,
        cents: i64,
        account_id: AccountId,
    ) -> TransactionRecord {
        TransactionRecord {
            account_id: Some(account_id),
            ..transaction(category, cents)
        }
    }

    fn account(
        id: AccountId,
        kind: AccountKind,
        budget_treatment: BudgetTreatment,
        cents: i64,
    ) -> AccountProfile {
        AccountProfile {
            id,
            name: "Demo account".to_string(),
            kind,
            budget_treatment,
            status: AccountStatus::Open,
            current_balance: MoneyAmount::cad_cents(cents),
        }
    }

    fn account_id(value: &str) -> AccountId {
        AccountId::new(value).expect("valid account id")
    }
}
