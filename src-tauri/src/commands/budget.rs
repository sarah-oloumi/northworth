use crate::domain::account::AccountProfile;
use crate::domain::budget::{
    calculate_budget_month as calculate_budget_month_domain,
    summarize_for_budget_pies_with_accounts, BudgetMonthInput, BudgetMonthSummary, BudgetSummary,
};
use crate::domain::transaction::TransactionRecord;

#[tauri::command]
pub fn calculate_budget_month(input: BudgetMonthInput) -> BudgetMonthSummary {
    calculate_budget_month_domain(&input)
}

#[tauri::command]
pub fn summarize_budget_pies(
    transactions: Vec<TransactionRecord>,
    accounts: Vec<AccountProfile>,
) -> BudgetSummary {
    summarize_for_budget_pies_with_accounts(&transactions, &accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::account::{AccountId, AccountKind, AccountStatus, BudgetTreatment};
    use crate::domain::budget::{BudgetAssignment, BudgetCategoryBalance, BudgetSlice};
    use crate::domain::money::MoneyAmount;
    use crate::domain::transaction::{ImportReviewStatus, TransactionCategory, TransactionDate};

    #[test]
    fn calculates_budget_month_through_command() {
        let summary = calculate_budget_month(BudgetMonthInput {
            starting_to_budget: MoneyAmount::cad_cents(0),
            starting_category_balances: vec![BudgetCategoryBalance {
                category: TransactionCategory::Groceries,
                amount: MoneyAmount::cad_cents(5_000),
            }],
            assignments: vec![BudgetAssignment {
                category: TransactionCategory::Groceries,
                amount: MoneyAmount::cad_cents(20_000),
            }],
            transactions: vec![transaction(TransactionCategory::Groceries, -8_000, None)],
            held_for_future_month: MoneyAmount::cad_cents(0),
            rollover_overspending_categories: vec![],
        });

        assert_eq!(summary.assigned, MoneyAmount::cad_cents(20_000));
        assert_eq!(summary.spent, MoneyAmount::cad_cents(8_000));
        assert_eq!(
            summary.categories[0].available,
            MoneyAmount::cad_cents(17_000)
        );
    }

    #[test]
    fn summarizes_budget_pies_through_command() {
        let chequing = account_id("chequing");
        let mortgage = account_id("mortgage");
        let accounts = vec![
            account(chequing.clone(), BudgetTreatment::OnBudget),
            account(mortgage.clone(), BudgetTreatment::OffBudgetTracking),
        ];
        let transactions = vec![
            transaction(TransactionCategory::Groceries, -10_000, Some(chequing)),
            transaction(TransactionCategory::Debt, -200_000, Some(mortgage)),
        ];

        let summary = summarize_budget_pies(transactions, accounts);

        assert_eq!(
            summary.outflow,
            vec![BudgetSlice {
                category: TransactionCategory::Groceries,
                amount: MoneyAmount::cad_cents(10_000),
                transaction_count: 1,
            }]
        );
    }

    fn transaction(
        category: TransactionCategory,
        cents: i64,
        account_id: Option<AccountId>,
    ) -> TransactionRecord {
        TransactionRecord {
            source_row: 2,
            date: TransactionDate {
                year: 2026,
                month: 1,
                day: 1,
            },
            description: "Demo transaction".to_string(),
            amount: MoneyAmount::cad_cents(cents),
            account_id,
            account_name: None,
            category: Some(category),
            import_status: ImportReviewStatus::PendingReview,
        }
    }

    fn account(id: AccountId, budget_treatment: BudgetTreatment) -> AccountProfile {
        AccountProfile {
            id,
            name: "Demo account".to_string(),
            kind: AccountKind::Chequing,
            budget_treatment,
            status: AccountStatus::Open,
            current_balance: MoneyAmount::cad_cents(0),
        }
    }

    fn account_id(value: &str) -> AccountId {
        AccountId::new(value).expect("valid account id")
    }
}
