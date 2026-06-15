use super::account::AccountId;
use super::money::MoneyAmount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub source_row: usize,
    pub date: TransactionDate,
    pub description: String,
    pub amount: MoneyAmount,
    pub account_id: Option<AccountId>,
    pub account_name: Option<String>,
    pub category: Option<TransactionCategory>,
    pub import_status: ImportReviewStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TransactionDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl TransactionDate {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, TransactionValidationError> {
        if month == 0 || month > 12 {
            return Err(TransactionValidationError::InvalidMonth(month));
        }

        let max_day = days_in_month(year, month);

        if day == 0 || day > max_day {
            return Err(TransactionValidationError::InvalidDay { month, day });
        }

        Ok(Self { year, month, day })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DateFormat {
    IsoYearMonthDay,
    MonthDayYear,
    DayMonthYear,
}

impl DateFormat {
    pub fn parse(&self, value: &str) -> Result<TransactionDate, TransactionValidationError> {
        let normalized = value.trim().replace('/', "-");
        let parts = normalized.split('-').collect::<Vec<_>>();

        if parts.len() != 3 {
            return Err(TransactionValidationError::InvalidDate(value.to_string()));
        }

        let first = parse_date_part(parts[0], value)?;
        let second = parse_date_part(parts[1], value)?;
        let third = parse_date_part(parts[2], value)?;

        match self {
            DateFormat::IsoYearMonthDay => TransactionDate::new(first, second as u8, third as u8),
            DateFormat::MonthDayYear => TransactionDate::new(third, first as u8, second as u8),
            DateFormat::DayMonthYear => TransactionDate::new(third, second as u8, first as u8),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TransactionCategory {
    Bills,
    Needs,
    Wants,
    Groceries,
    Gas,
    Debt,
    Subscriptions,
    Housing,
    Auto,
    Insurance,
    Taxes,
    Investments,
    Income,
    Transfers,
    Uncategorized,
    Other(String),
}

impl TransactionCategory {
    pub fn from_import_value(value: &str) -> Option<Self> {
        let normalized = value.trim().to_lowercase();

        if normalized.is_empty() {
            return None;
        }

        Some(match normalized.as_str() {
            "bill" | "bills" | "utility" | "utilities" => Self::Bills,
            "need" | "needs" => Self::Needs,
            "want" | "wants" => Self::Wants,
            "grocery" | "groceries" | "supermarket" => Self::Groceries,
            "gas" | "fuel" => Self::Gas,
            "debt" => Self::Debt,
            "subscription" | "subscriptions" => Self::Subscriptions,
            "housing" | "home" | "mortgage" | "rent" => Self::Housing,
            "auto" | "car" | "vehicle" => Self::Auto,
            "insurance" => Self::Insurance,
            "tax" | "taxes" => Self::Taxes,
            "investment" | "investments" => Self::Investments,
            "income" | "payroll" | "salary" => Self::Income,
            "transfer" | "transfers" => Self::Transfers,
            "uncategorized" => Self::Uncategorized,
            _ => Self::Other(value.trim().to_string()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImportReviewStatus {
    PendingReview,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionValidationError {
    InvalidDate(String),
    InvalidMonth(u8),
    InvalidDay { month: u8, day: u8 },
}

fn parse_date_part(value: &str, original: &str) -> Result<u16, TransactionValidationError> {
    value
        .trim()
        .parse::<u16>()
        .map_err(|_| TransactionValidationError::InvalidDate(original.to_string()))
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
    fn parses_supported_date_formats() {
        assert_eq!(
            DateFormat::IsoYearMonthDay.parse("2026-06-14"),
            Ok(TransactionDate {
                year: 2026,
                month: 6,
                day: 14
            })
        );
        assert_eq!(
            DateFormat::MonthDayYear.parse("06/14/2026"),
            Ok(TransactionDate {
                year: 2026,
                month: 6,
                day: 14
            })
        );
        assert_eq!(
            DateFormat::DayMonthYear.parse("14/06/2026"),
            Ok(TransactionDate {
                year: 2026,
                month: 6,
                day: 14
            })
        );
    }

    #[test]
    fn rejects_invalid_dates() {
        assert_eq!(
            DateFormat::IsoYearMonthDay.parse("2026-02-30"),
            Err(TransactionValidationError::InvalidDay { month: 2, day: 30 })
        );
    }

    #[test]
    fn maps_import_categories() {
        assert_eq!(
            TransactionCategory::from_import_value("Mortgage"),
            Some(TransactionCategory::Housing)
        );
        assert_eq!(
            TransactionCategory::from_import_value("Mystery"),
            Some(TransactionCategory::Other("Mystery".to_string()))
        );
        assert_eq!(TransactionCategory::from_import_value(" "), None);
    }
}
