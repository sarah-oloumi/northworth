use super::account::{AccountId, AccountProfile, BudgetTreatment};
use super::money::MoneyAmount;
use super::transaction::{TransactionCategory, TransactionRecord};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetMonthInput {
    pub starting_to_budget: MoneyAmount,
    pub starting_category_balances: Vec<BudgetCategoryBalance>,
    pub assignments: Vec<BudgetAssignment>,
    pub transactions: Vec<TransactionRecord>,
    pub held_for_future_month: MoneyAmount,
    pub rollover_overspending_categories: Vec<TransactionCategory>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetCategoryBalance {
    pub category: TransactionCategory,
    pub amount: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetAssignment {
    pub category: TransactionCategory,
    pub amount: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetMonthSummary {
    pub categories: Vec<BudgetCategoryMonth>,
    pub inflow: MoneyAmount,
    pub assigned: MoneyAmount,
    pub spent: MoneyAmount,
    pub held_for_future_month: MoneyAmount,
    pub to_budget: MoneyAmount,
    pub overspending_deduction_next_month: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetCategoryMonth {
    pub category: TransactionCategory,
    pub starting_balance: MoneyAmount,
    pub assigned: MoneyAmount,
    pub activity: MoneyAmount,
    pub available: MoneyAmount,
    pub rollover_to_next_month: MoneyAmount,
    pub overspent: MoneyAmount,
}

pub fn calculate_budget_month(input: &BudgetMonthInput) -> BudgetMonthSummary {
    let mut categories = BTreeMap::<TransactionCategory, BudgetCategoryAccumulator>::new();
    let mut inflow_cents = 0;

    for balance in &input.starting_category_balances {
        categories
            .entry(balance.category.clone())
            .or_default()
            .starting_balance_cents += balance.amount.cents;
    }

    for assignment in &input.assignments {
        categories
            .entry(assignment.category.clone())
            .or_default()
            .assigned_cents += assignment.amount.cents;
    }

    for transaction in &input.transactions {
        if transaction.amount.is_inflow() {
            inflow_cents += transaction.amount.cents;
            continue;
        }

        if transaction.amount.is_outflow() {
            let category = transaction
                .category
                .clone()
                .unwrap_or(TransactionCategory::Uncategorized);

            categories.entry(category).or_default().activity_cents += transaction.amount.cents;
        }
    }

    let rollover_overspending = input
        .rollover_overspending_categories
        .iter()
        .cloned()
        .collect::<HashSet<_>>();
    let mut category_months = Vec::new();
    let mut overspending_deduction_next_month_cents = 0;

    for (category, accumulator) in categories {
        let available_cents = accumulator.starting_balance_cents
            + accumulator.assigned_cents
            + accumulator.activity_cents;
        let rolls_over_negative = rollover_overspending.contains(&category);
        let overspent_cents = if available_cents < 0 && !rolls_over_negative {
            available_cents.abs()
        } else {
            0
        };
        let rollover_to_next_month_cents = if overspent_cents > 0 {
            0
        } else {
            available_cents
        };

        overspending_deduction_next_month_cents += overspent_cents;

        category_months.push(BudgetCategoryMonth {
            category,
            starting_balance: MoneyAmount::cad_cents(accumulator.starting_balance_cents),
            assigned: MoneyAmount::cad_cents(accumulator.assigned_cents),
            activity: MoneyAmount::cad_cents(accumulator.activity_cents),
            available: MoneyAmount::cad_cents(available_cents),
            rollover_to_next_month: MoneyAmount::cad_cents(rollover_to_next_month_cents),
            overspent: MoneyAmount::cad_cents(overspent_cents),
        });
    }

    category_months.sort_by(|left, right| left.category.cmp(&right.category));

    let assigned_cents = input
        .assignments
        .iter()
        .map(|assignment| assignment.amount.cents)
        .sum::<i64>();
    let spent_cents = input
        .transactions
        .iter()
        .filter(|transaction| transaction.amount.is_outflow())
        .map(|transaction| transaction.amount.cents.abs())
        .sum::<i64>();
    let to_budget_cents = input.starting_to_budget.cents + inflow_cents
        - assigned_cents
        - input.held_for_future_month.cents;

    BudgetMonthSummary {
        categories: category_months,
        inflow: MoneyAmount::cad_cents(inflow_cents),
        assigned: MoneyAmount::cad_cents(assigned_cents),
        spent: MoneyAmount::cad_cents(spent_cents),
        held_for_future_month: input.held_for_future_month.clone(),
        to_budget: MoneyAmount::cad_cents(to_budget_cents),
        overspending_deduction_next_month: MoneyAmount::cad_cents(
            overspending_deduction_next_month_cents,
        ),
    }
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

#[derive(Debug, Default)]
struct BudgetCategoryAccumulator {
    starting_balance_cents: i64,
    assigned_cents: i64,
    activity_cents: i64,
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

    #[test]
    fn calculates_budget_month_from_available_money_and_activity() {
        let input = BudgetMonthInput {
            starting_to_budget: MoneyAmount::cad_cents(15_000),
            starting_category_balances: vec![BudgetCategoryBalance {
                category: TransactionCategory::Groceries,
                amount: MoneyAmount::cad_cents(2_000),
            }],
            assignments: vec![
                BudgetAssignment {
                    category: TransactionCategory::Groceries,
                    amount: MoneyAmount::cad_cents(60_000),
                },
                BudgetAssignment {
                    category: TransactionCategory::Gas,
                    amount: MoneyAmount::cad_cents(20_000),
                },
            ],
            transactions: vec![
                transaction(TransactionCategory::Income, 100_000),
                transaction(TransactionCategory::Groceries, -25_000),
                transaction(TransactionCategory::Gas, -8_000),
            ],
            held_for_future_month: MoneyAmount::cad_cents(10_000),
            rollover_overspending_categories: vec![],
        };

        let summary = calculate_budget_month(&input);

        assert_eq!(summary.inflow, MoneyAmount::cad_cents(100_000));
        assert_eq!(summary.assigned, MoneyAmount::cad_cents(80_000));
        assert_eq!(summary.spent, MoneyAmount::cad_cents(33_000));
        assert_eq!(summary.to_budget, MoneyAmount::cad_cents(25_000));
        assert_eq!(
            summary.overspending_deduction_next_month,
            MoneyAmount::cad_cents(0)
        );
        assert_eq!(
            summary.categories,
            vec![
                BudgetCategoryMonth {
                    category: TransactionCategory::Groceries,
                    starting_balance: MoneyAmount::cad_cents(2_000),
                    assigned: MoneyAmount::cad_cents(60_000),
                    activity: MoneyAmount::cad_cents(-25_000),
                    available: MoneyAmount::cad_cents(37_000),
                    rollover_to_next_month: MoneyAmount::cad_cents(37_000),
                    overspent: MoneyAmount::cad_cents(0),
                },
                BudgetCategoryMonth {
                    category: TransactionCategory::Gas,
                    starting_balance: MoneyAmount::cad_cents(0),
                    assigned: MoneyAmount::cad_cents(20_000),
                    activity: MoneyAmount::cad_cents(-8_000),
                    available: MoneyAmount::cad_cents(12_000),
                    rollover_to_next_month: MoneyAmount::cad_cents(12_000),
                    overspent: MoneyAmount::cad_cents(0),
                },
            ]
        );
    }

    #[test]
    fn resets_ordinary_overspending_and_deducts_it_next_month() {
        let input = BudgetMonthInput {
            starting_to_budget: MoneyAmount::cad_cents(0),
            starting_category_balances: vec![],
            assignments: vec![BudgetAssignment {
                category: TransactionCategory::Groceries,
                amount: MoneyAmount::cad_cents(10_000),
            }],
            transactions: vec![transaction(TransactionCategory::Groceries, -12_500)],
            held_for_future_month: MoneyAmount::cad_cents(0),
            rollover_overspending_categories: vec![],
        };

        let summary = calculate_budget_month(&input);

        assert_eq!(
            summary.overspending_deduction_next_month,
            MoneyAmount::cad_cents(2_500)
        );
        assert_eq!(
            summary.categories[0].available,
            MoneyAmount::cad_cents(-2_500)
        );
        assert_eq!(
            summary.categories[0].rollover_to_next_month,
            MoneyAmount::cad_cents(0)
        );
    }

    #[test]
    fn keeps_negative_balances_for_reimbursable_rollover_categories() {
        let input = BudgetMonthInput {
            starting_to_budget: MoneyAmount::cad_cents(0),
            starting_category_balances: vec![],
            assignments: vec![],
            transactions: vec![transaction(
                TransactionCategory::Other("Reimbursable".to_string()),
                -5_000,
            )],
            held_for_future_month: MoneyAmount::cad_cents(0),
            rollover_overspending_categories: vec![TransactionCategory::Other(
                "Reimbursable".to_string(),
            )],
        };

        let summary = calculate_budget_month(&input);

        assert_eq!(
            summary.overspending_deduction_next_month,
            MoneyAmount::cad_cents(0)
        );
        assert_eq!(
            summary.categories[0].rollover_to_next_month,
            MoneyAmount::cad_cents(-5_000)
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
