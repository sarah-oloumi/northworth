use crate::storage::{persistence_readiness, PersistenceReadiness};

#[tauri::command]
pub fn get_persistence_readiness() -> PersistenceReadiness {
    persistence_readiness(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::DurablePersistenceStatus;

    #[test]
    fn command_reports_private_persistence_as_blocked_for_now() {
        let readiness = get_persistence_readiness();

        assert_eq!(
            readiness.private_persistence,
            DurablePersistenceStatus::BlockedPendingEncryptionDecision
        );
        assert!(!readiness.can_save_private_financial_data);
    }
}
