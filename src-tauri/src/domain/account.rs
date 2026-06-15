use super::money::MoneyAmount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AccountId(String);

impl AccountId {
    pub fn new(value: impl Into<String>) -> Result<Self, AccountValidationError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(AccountValidationError::EmptyAccountId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountProfile {
    pub id: AccountId,
    pub name: String,
    pub kind: AccountKind,
    pub budget_treatment: BudgetTreatment,
    pub status: AccountStatus,
    pub current_balance: MoneyAmount,
}

impl AccountProfile {
    pub fn validate(&self) -> Result<(), AccountValidationError> {
        if self.id.as_str().trim().is_empty() {
            return Err(AccountValidationError::EmptyAccountId);
        }

        if self.name.trim().is_empty() {
            return Err(AccountValidationError::EmptyAccountName {
                account_id: self.id.clone(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountKind {
    Chequing,
    Savings,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Loan,
    TaxableBrokerage,
    RegisteredInvestment,
    Property,
    Vehicle,
    Cash,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetTreatment {
    OnBudget,
    OffBudgetTracking,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountValidationError {
    EmptyAccountId,
    EmptyAccountName { account_id: AccountId },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_named_accounts() {
        let account = AccountProfile {
            id: account_id("chequing"),
            name: "Demo chequing".to_string(),
            kind: AccountKind::Chequing,
            budget_treatment: BudgetTreatment::OnBudget,
            status: AccountStatus::Open,
            current_balance: MoneyAmount::cad_cents(100_00),
        };

        assert_eq!(account.validate(), Ok(()));
    }

    #[test]
    fn rejects_empty_account_names() {
        let account = AccountProfile {
            id: account_id("mortgage"),
            name: " ".to_string(),
            kind: AccountKind::Mortgage,
            budget_treatment: BudgetTreatment::OffBudgetTracking,
            status: AccountStatus::Open,
            current_balance: MoneyAmount::cad_cents(-500_000_00),
        };

        assert_eq!(
            account.validate(),
            Err(AccountValidationError::EmptyAccountName {
                account_id: account_id("mortgage")
            })
        );
    }

    #[test]
    fn normalizes_account_ids() {
        assert_eq!(account_id(" chequing ").as_str(), "chequing");
    }

    #[test]
    fn rejects_empty_account_ids() {
        assert_eq!(
            AccountId::new(" "),
            Err(AccountValidationError::EmptyAccountId)
        );
    }

    fn account_id(value: &str) -> AccountId {
        AccountId::new(value).expect("valid account id")
    }
}
