use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoredDataClass {
    PublicSourceCache,
    UserPrivateFinancialData,
    LocalSecret,
    DerivedPrivatePlanningData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurablePersistenceStatus {
    Allowed,
    BlockedPendingEncryptionDecision,
    SecretStoreOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageBoundaryPolicy {
    pub data_class: StoredDataClass,
    pub durable_persistence: DurablePersistenceStatus,
    pub requires_encryption_at_rest: bool,
    pub requires_user_action_for_export: bool,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistenceReadiness {
    pub private_persistence: DurablePersistenceStatus,
    pub can_save_private_financial_data: bool,
    pub policies: Vec<StorageBoundaryPolicy>,
}

pub fn persistence_readiness(encryption_decision_accepted: bool) -> PersistenceReadiness {
    let private_persistence = if encryption_decision_accepted {
        DurablePersistenceStatus::Allowed
    } else {
        DurablePersistenceStatus::BlockedPendingEncryptionDecision
    };

    PersistenceReadiness {
        private_persistence,
        can_save_private_financial_data: encryption_decision_accepted,
        policies: storage_boundary_policies(encryption_decision_accepted),
    }
}

pub fn storage_boundary_policies(encryption_decision_accepted: bool) -> Vec<StorageBoundaryPolicy> {
    vec![
        StorageBoundaryPolicy {
            data_class: StoredDataClass::PublicSourceCache,
            durable_persistence: DurablePersistenceStatus::Allowed,
            requires_encryption_at_rest: false,
            requires_user_action_for_export: false,
            notes: "Public source cache may be stored locally with retrieval timestamps and stale-state metadata."
                .to_string(),
        },
        StorageBoundaryPolicy {
            data_class: StoredDataClass::UserPrivateFinancialData,
            durable_persistence: private_status(encryption_decision_accepted),
            requires_encryption_at_rest: true,
            requires_user_action_for_export: true,
            notes: "Imported transactions, accounts, holdings, tax facts, and balances require accepted encrypted persistence before durable save."
                .to_string(),
        },
        StorageBoundaryPolicy {
            data_class: StoredDataClass::LocalSecret,
            durable_persistence: DurablePersistenceStatus::SecretStoreOnly,
            requires_encryption_at_rest: true,
            requires_user_action_for_export: true,
            notes: "API keys, provider tokens, local model credentials, and encryption keys must use OS secret storage or an accepted equivalent."
                .to_string(),
        },
        StorageBoundaryPolicy {
            data_class: StoredDataClass::DerivedPrivatePlanningData,
            durable_persistence: private_status(encryption_decision_accepted),
            requires_encryption_at_rest: true,
            requires_user_action_for_export: true,
            notes: "Budgets, projections, scenarios, risk profiles, and audit trails derived from private inputs are private data."
                .to_string(),
        },
    ]
}

fn private_status(encryption_decision_accepted: bool) -> DurablePersistenceStatus {
    if encryption_decision_accepted {
        DurablePersistenceStatus::Allowed
    } else {
        DurablePersistenceStatus::BlockedPendingEncryptionDecision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_private_persistence_until_encryption_decision_is_accepted() {
        let readiness = persistence_readiness(false);

        assert_eq!(
            readiness.private_persistence,
            DurablePersistenceStatus::BlockedPendingEncryptionDecision
        );
        assert!(!readiness.can_save_private_financial_data);
        assert_eq!(
            private_policy(&readiness).durable_persistence,
            DurablePersistenceStatus::BlockedPendingEncryptionDecision
        );
    }

    #[test]
    fn allows_public_source_cache_without_private_encryption_gate() {
        let readiness = persistence_readiness(false);
        let public_cache = readiness
            .policies
            .iter()
            .find(|policy| policy.data_class == StoredDataClass::PublicSourceCache)
            .expect("public source cache policy exists");

        assert_eq!(
            public_cache.durable_persistence,
            DurablePersistenceStatus::Allowed
        );
        assert!(!public_cache.requires_encryption_at_rest);
    }

    #[test]
    fn marks_private_persistence_ready_after_encryption_decision() {
        let readiness = persistence_readiness(true);

        assert_eq!(
            readiness.private_persistence,
            DurablePersistenceStatus::Allowed
        );
        assert!(readiness.can_save_private_financial_data);
        assert_eq!(
            private_policy(&readiness).durable_persistence,
            DurablePersistenceStatus::Allowed
        );
    }

    #[test]
    fn requires_secrets_to_use_secret_storage_only() {
        let readiness = persistence_readiness(true);
        let secret_policy = readiness
            .policies
            .iter()
            .find(|policy| policy.data_class == StoredDataClass::LocalSecret)
            .expect("secret policy exists");

        assert_eq!(
            secret_policy.durable_persistence,
            DurablePersistenceStatus::SecretStoreOnly
        );
        assert!(secret_policy.requires_encryption_at_rest);
    }

    fn private_policy(readiness: &PersistenceReadiness) -> &StorageBoundaryPolicy {
        readiness
            .policies
            .iter()
            .find(|policy| policy.data_class == StoredDataClass::UserPrivateFinancialData)
            .expect("private policy exists")
    }
}
