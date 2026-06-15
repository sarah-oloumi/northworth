use super::transaction::TransactionDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketDataRefreshCadence {
    Monthly,
    Quarterly,
    Biannual,
    Yearly,
}

impl MarketDataRefreshCadence {
    pub fn refresh_after_days(self) -> i64 {
        match self {
            Self::Monthly => 31,
            Self::Quarterly => 92,
            Self::Biannual => 183,
            Self::Yearly => 366,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Monthly => "Monthly",
            Self::Quarterly => "Quarterly",
            Self::Biannual => "Biannual",
            Self::Yearly => "Yearly",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Monthly => {
                "Refresh prices and issuer facts when a cache is more than 31 days old."
            }
            Self::Quarterly => "Refresh when a cache is more than 92 days old.",
            Self::Biannual => "Refresh when a cache is more than 183 days old.",
            Self::Yearly => "Refresh when a cache is more than 366 days old.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketDataRefreshPolicy {
    pub cadence: MarketDataRefreshCadence,
    pub label: String,
    pub refresh_after_days: i64,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketDataFreshnessStatus {
    Current,
    Stale,
    FutureDated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketDataFreshness {
    pub cadence: MarketDataRefreshCadence,
    pub status: MarketDataFreshnessStatus,
    pub age_days: i64,
    pub refresh_after_days: i64,
    pub refreshed_at: TransactionDate,
    pub as_of: TransactionDate,
}

pub fn refresh_policies() -> Vec<MarketDataRefreshPolicy> {
    [
        MarketDataRefreshCadence::Monthly,
        MarketDataRefreshCadence::Quarterly,
        MarketDataRefreshCadence::Biannual,
        MarketDataRefreshCadence::Yearly,
    ]
    .into_iter()
    .map(|cadence| MarketDataRefreshPolicy {
        cadence,
        label: cadence.label().to_string(),
        refresh_after_days: cadence.refresh_after_days(),
        description: cadence.description().to_string(),
    })
    .collect()
}

pub fn assess_freshness(
    cadence: MarketDataRefreshCadence,
    refreshed_at: TransactionDate,
    as_of: TransactionDate,
) -> MarketDataFreshness {
    let age_days = days_between(refreshed_at, as_of);
    let refresh_after_days = cadence.refresh_after_days();
    let status = if age_days < 0 {
        MarketDataFreshnessStatus::FutureDated
    } else if age_days > refresh_after_days {
        MarketDataFreshnessStatus::Stale
    } else {
        MarketDataFreshnessStatus::Current
    };

    MarketDataFreshness {
        cadence,
        status,
        age_days: age_days.max(0),
        refresh_after_days,
        refreshed_at,
        as_of,
    }
}

fn days_between(start: TransactionDate, end: TransactionDate) -> i64 {
    day_number(end) - day_number(start)
}

fn day_number(date: TransactionDate) -> i64 {
    let mut days = 0;

    for year in 0..date.year {
        days += if is_leap_year(year) { 366 } else { 365 };
    }

    for month in 1..date.month {
        days += days_in_month(date.year, month) as i64;
    }

    days + date.day as i64
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_supported_refresh_policies() {
        let policies = refresh_policies();

        assert_eq!(policies.len(), 4);
        assert_eq!(policies[0].cadence, MarketDataRefreshCadence::Monthly);
        assert_eq!(policies[0].refresh_after_days, 31);
    }

    #[test]
    fn marks_cache_current_within_cadence() {
        let freshness = assess_freshness(
            MarketDataRefreshCadence::Quarterly,
            date(2026, 4, 1),
            date(2026, 6, 15),
        );

        assert_eq!(freshness.status, MarketDataFreshnessStatus::Current);
        assert_eq!(freshness.age_days, 75);
    }

    #[test]
    fn marks_cache_stale_after_cadence() {
        let freshness = assess_freshness(
            MarketDataRefreshCadence::Monthly,
            date(2026, 5, 1),
            date(2026, 6, 15),
        );

        assert_eq!(freshness.status, MarketDataFreshnessStatus::Stale);
        assert_eq!(freshness.age_days, 45);
    }

    #[test]
    fn handles_future_dated_provider_cache() {
        let freshness = assess_freshness(
            MarketDataRefreshCadence::Monthly,
            date(2026, 7, 1),
            date(2026, 6, 15),
        );

        assert_eq!(freshness.status, MarketDataFreshnessStatus::FutureDated);
        assert_eq!(freshness.age_days, 0);
    }

    fn date(year: u16, month: u8, day: u8) -> TransactionDate {
        TransactionDate { year, month, day }
    }
}
