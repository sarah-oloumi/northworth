use crate::domain::market_data::{
    assess_freshness, refresh_policies, MarketDataFreshness, MarketDataRefreshCadence,
    MarketDataRefreshPolicy,
};
use crate::domain::transaction::TransactionDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketDataFreshnessInput {
    pub cadence: MarketDataRefreshCadence,
    pub refreshed_at: TransactionDate,
    pub as_of: TransactionDate,
}

#[tauri::command]
pub fn list_market_data_refresh_policies() -> Vec<MarketDataRefreshPolicy> {
    refresh_policies()
}

#[tauri::command]
pub fn assess_market_data_freshness(input: MarketDataFreshnessInput) -> MarketDataFreshness {
    assess_freshness(input.cadence, input.refreshed_at, input.as_of)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::market_data::MarketDataFreshnessStatus;

    #[test]
    fn lists_refresh_policies_through_command() {
        let policies = list_market_data_refresh_policies();

        assert_eq!(policies.len(), 4);
        assert_eq!(policies[1].cadence, MarketDataRefreshCadence::Quarterly);
    }

    #[test]
    fn assesses_freshness_through_command() {
        let freshness = assess_market_data_freshness(MarketDataFreshnessInput {
            cadence: MarketDataRefreshCadence::Monthly,
            refreshed_at: date(2026, 5, 1),
            as_of: date(2026, 6, 15),
        });

        assert_eq!(freshness.status, MarketDataFreshnessStatus::Stale);
        assert_eq!(freshness.age_days, 45);
    }

    fn date(year: u16, month: u8, day: u8) -> TransactionDate {
        TransactionDate { year, month, day }
    }
}
