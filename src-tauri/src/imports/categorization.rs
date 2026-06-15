use crate::domain::transaction::{TransactionCategory, TransactionRecord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryRule {
    pub name: String,
    pub priority: u16,
    pub match_text: String,
    pub category: TransactionCategory,
}

impl CategoryRule {
    pub fn contains(
        name: impl Into<String>,
        priority: u16,
        match_text: impl Into<String>,
        category: TransactionCategory,
    ) -> Self {
        Self {
            name: name.into(),
            priority,
            match_text: match_text.into(),
            category,
        }
    }

    fn matches(&self, transaction: &TransactionRecord) -> bool {
        transaction
            .description
            .to_lowercase()
            .contains(&self.match_text.to_lowercase())
    }
}

pub fn apply_category_rules(
    transactions: &mut [TransactionRecord],
    rules: &[CategoryRule],
) -> CategoryRuleApplicationSummary {
    let mut sorted_rules = rules.to_vec();
    sorted_rules.sort_by(|left, right| {
        left.priority
            .cmp(&right.priority)
            .then_with(|| left.name.cmp(&right.name))
    });

    let mut categorized_count = 0;

    for transaction in transactions {
        if transaction.category.is_some() {
            continue;
        }

        if let Some(rule) = sorted_rules.iter().find(|rule| rule.matches(transaction)) {
            transaction.category = Some(rule.category.clone());
            categorized_count += 1;
        }
    }

    CategoryRuleApplicationSummary { categorized_count }
}

pub fn default_budget_rules() -> Vec<CategoryRule> {
    vec![
        CategoryRule::contains("grocery", 100, "grocery", TransactionCategory::Groceries),
        CategoryRule::contains(
            "supermarket",
            100,
            "supermarket",
            TransactionCategory::Groceries,
        ),
        CategoryRule::contains("fuel", 100, "fuel", TransactionCategory::Gas),
        CategoryRule::contains("gas", 110, "gas", TransactionCategory::Gas),
        CategoryRule::contains("hydro", 100, "hydro", TransactionCategory::Bills),
        CategoryRule::contains("internet", 100, "internet", TransactionCategory::Bills),
        CategoryRule::contains("phone", 100, "phone", TransactionCategory::Bills),
        CategoryRule::contains("payroll", 100, "payroll", TransactionCategory::Income),
        CategoryRule::contains("salary", 100, "salary", TransactionCategory::Income),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryRuleApplicationSummary {
    pub categorized_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::money::MoneyAmount;
    use crate::domain::transaction::{ImportReviewStatus, TransactionDate};

    #[test]
    fn applies_rules_to_uncategorized_transactions() {
        let mut transactions = vec![
            transaction("Demo Grocery Store", -10_00, None),
            transaction("Demo Gas Bar", -20_00, None),
            transaction("Demo Payroll", 100_00, None),
        ];

        let summary = apply_category_rules(&mut transactions, &default_budget_rules());

        assert_eq!(summary.categorized_count, 3);
        assert_eq!(
            transactions[0].category,
            Some(TransactionCategory::Groceries)
        );
        assert_eq!(transactions[1].category, Some(TransactionCategory::Gas));
        assert_eq!(transactions[2].category, Some(TransactionCategory::Income));
    }

    #[test]
    fn preserves_existing_categories() {
        let mut transactions = vec![transaction(
            "Demo Grocery Store",
            -10_00,
            Some(TransactionCategory::Wants),
        )];

        let summary = apply_category_rules(&mut transactions, &default_budget_rules());

        assert_eq!(summary.categorized_count, 0);
        assert_eq!(transactions[0].category, Some(TransactionCategory::Wants));
    }

    fn transaction(
        description: &str,
        cents: i64,
        category: Option<TransactionCategory>,
    ) -> TransactionRecord {
        TransactionRecord {
            source_row: 2,
            date: TransactionDate {
                year: 2026,
                month: 6,
                day: 1,
            },
            description: description.to_string(),
            amount: MoneyAmount::cad_cents(cents),
            account_id: None,
            account_name: Some("Demo account".to_string()),
            category,
            import_status: ImportReviewStatus::PendingReview,
        }
    }
}
