use crate::domain::money::MoneyAmount;
use crate::domain::transaction::{
    DateFormat, ImportReviewStatus, TransactionCategory, TransactionRecord,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CsvImportMapping {
    pub date_column: String,
    pub date_format: DateFormat,
    pub description_column: String,
    pub amount_mapping: CsvAmountMapping,
    pub account_column: Option<String>,
    pub category_column: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CsvAmountMapping {
    Single {
        column: String,
        convention: AmountSignConvention,
    },
    Split {
        debit_column: String,
        credit_column: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmountSignConvention {
    NegativeIsOutflow,
    PositiveIsOutflow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CsvImportPreview {
    pub transactions: Vec<TransactionRecord>,
    pub errors: Vec<CsvImportError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CsvImportError {
    pub row: Option<usize>,
    pub column: Option<String>,
    pub message: String,
}

pub fn preview_transactions(csv_contents: &str, mapping: &CsvImportMapping) -> CsvImportPreview {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(Cursor::new(csv_contents));

    let headers = match reader.headers() {
        Ok(headers) => headers.clone(),
        Err(error) => {
            return CsvImportPreview {
                transactions: vec![],
                errors: vec![CsvImportError {
                    row: None,
                    column: None,
                    message: format!("Unable to read CSV headers: {error}"),
                }],
            };
        }
    };

    let column_index = build_column_index(&headers);
    let mapping_error = validate_mapping(mapping, &column_index);

    if !mapping_error.is_empty() {
        return CsvImportPreview {
            transactions: vec![],
            errors: mapping_error,
        };
    }

    let mut transactions = Vec::new();
    let mut errors = Vec::new();

    for (zero_based_record_index, record_result) in reader.records().enumerate() {
        let source_row = zero_based_record_index + 2;

        match record_result {
            Ok(record) => match parse_record(&record, source_row, mapping, &column_index) {
                Ok(transaction) => transactions.push(transaction),
                Err(mut row_errors) => errors.append(&mut row_errors),
            },
            Err(error) => errors.push(CsvImportError {
                row: Some(source_row),
                column: None,
                message: format!("Unable to parse CSV row: {error}"),
            }),
        }
    }

    CsvImportPreview {
        transactions,
        errors,
    }
}

fn build_column_index(headers: &csv::StringRecord) -> HashMap<String, usize> {
    headers
        .iter()
        .enumerate()
        .map(|(index, header)| (normalize_header(header), index))
        .collect()
}

fn validate_mapping(
    mapping: &CsvImportMapping,
    column_index: &HashMap<String, usize>,
) -> Vec<CsvImportError> {
    let mut errors = Vec::new();

    require_column(
        &mapping.date_column,
        "date_column",
        column_index,
        &mut errors,
    );
    require_column(
        &mapping.description_column,
        "description_column",
        column_index,
        &mut errors,
    );

    match &mapping.amount_mapping {
        CsvAmountMapping::Single { column, .. } => {
            require_column(column, "amount_mapping.column", column_index, &mut errors);
        }
        CsvAmountMapping::Split {
            debit_column,
            credit_column,
        } => {
            require_column(
                debit_column,
                "amount_mapping.debit_column",
                column_index,
                &mut errors,
            );
            require_column(
                credit_column,
                "amount_mapping.credit_column",
                column_index,
                &mut errors,
            );
        }
    }

    if let Some(column) = &mapping.account_column {
        require_column(column, "account_column", column_index, &mut errors);
    }

    if let Some(column) = &mapping.category_column {
        require_column(column, "category_column", column_index, &mut errors);
    }

    errors
}

fn require_column(
    column: &str,
    field_name: &str,
    column_index: &HashMap<String, usize>,
    errors: &mut Vec<CsvImportError>,
) {
    if !column_index.contains_key(&normalize_header(column)) {
        errors.push(CsvImportError {
            row: None,
            column: Some(column.to_string()),
            message: format!(
                "Mapping field `{field_name}` references missing CSV column `{column}`"
            ),
        });
    }
}

fn parse_record(
    record: &csv::StringRecord,
    source_row: usize,
    mapping: &CsvImportMapping,
    column_index: &HashMap<String, usize>,
) -> Result<TransactionRecord, Vec<CsvImportError>> {
    let mut errors = Vec::new();

    let date = match get_required_value(record, &mapping.date_column, column_index) {
        Some(value) => match mapping.date_format.parse(value) {
            Ok(date) => Some(date),
            Err(error) => {
                errors.push(CsvImportError {
                    row: Some(source_row),
                    column: Some(mapping.date_column.clone()),
                    message: format!("Invalid date `{value}`: {error:?}"),
                });
                None
            }
        },
        None => {
            errors.push(missing_value_error(source_row, &mapping.date_column));
            None
        }
    };

    let description = match get_required_value(record, &mapping.description_column, column_index) {
        Some(value) if !value.trim().is_empty() => Some(value.trim().to_string()),
        _ => {
            errors.push(missing_value_error(source_row, &mapping.description_column));
            None
        }
    };

    let amount = match parse_amount(record, source_row, &mapping.amount_mapping, column_index) {
        Ok(amount) => Some(amount),
        Err(mut amount_errors) => {
            errors.append(&mut amount_errors);
            None
        }
    };

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(TransactionRecord {
        source_row,
        date: date.expect("date checked before transaction construction"),
        description: description.expect("description checked before transaction construction"),
        amount: amount.expect("amount checked before transaction construction"),
        account_id: None,
        account_name: mapping
            .account_column
            .as_ref()
            .and_then(|column| get_optional_value(record, column, column_index))
            .map(str::to_string),
        category: mapping
            .category_column
            .as_ref()
            .and_then(|column| get_optional_value(record, column, column_index))
            .and_then(TransactionCategory::from_import_value),
        import_status: ImportReviewStatus::PendingReview,
    })
}

fn parse_amount(
    record: &csv::StringRecord,
    source_row: usize,
    amount_mapping: &CsvAmountMapping,
    column_index: &HashMap<String, usize>,
) -> Result<MoneyAmount, Vec<CsvImportError>> {
    match amount_mapping {
        CsvAmountMapping::Single { column, convention } => {
            let value = get_required_value(record, column, column_index)
                .ok_or_else(|| vec![missing_value_error(source_row, column)])?;
            let cents = parse_money_cents(value).map_err(|message| {
                vec![CsvImportError {
                    row: Some(source_row),
                    column: Some(column.clone()),
                    message,
                }]
            })?;

            let cents = match convention {
                AmountSignConvention::NegativeIsOutflow => cents,
                AmountSignConvention::PositiveIsOutflow => -cents,
            };

            Ok(MoneyAmount::cad_cents(cents))
        }
        CsvAmountMapping::Split {
            debit_column,
            credit_column,
        } => {
            let debit = get_optional_value(record, debit_column, column_index)
                .map(parse_money_cents)
                .transpose()
                .map_err(|message| {
                    vec![CsvImportError {
                        row: Some(source_row),
                        column: Some(debit_column.clone()),
                        message,
                    }]
                })?
                .unwrap_or(0);
            let credit = get_optional_value(record, credit_column, column_index)
                .map(parse_money_cents)
                .transpose()
                .map_err(|message| {
                    vec![CsvImportError {
                        row: Some(source_row),
                        column: Some(credit_column.clone()),
                        message,
                    }]
                })?
                .unwrap_or(0);

            if debit == 0 && credit == 0 {
                return Err(vec![CsvImportError {
                    row: Some(source_row),
                    column: None,
                    message: "Expected a debit or credit amount".to_string(),
                }]);
            }

            Ok(MoneyAmount::cad_cents(credit - debit))
        }
    }
}

fn parse_money_cents(value: &str) -> Result<i64, String> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Ok(0);
    }

    let is_parenthesized_negative = trimmed.starts_with('(') && trimmed.ends_with(')');
    let cleaned = trimmed
        .trim_start_matches('(')
        .trim_end_matches(')')
        .replace(['$', ','], "")
        .replace("CAD", "")
        .replace("cad", "")
        .trim()
        .to_string();

    let is_negative = is_parenthesized_negative || cleaned.starts_with('-');
    let unsigned = cleaned.trim_start_matches(['-', '+']);
    let parts = unsigned.split('.').collect::<Vec<_>>();

    if parts.len() > 2 || parts[0].is_empty() {
        return Err(format!("Invalid money amount `{value}`"));
    }

    let dollars = parts[0]
        .parse::<i64>()
        .map_err(|_| format!("Invalid money amount `{value}`"))?;
    let cents = match parts.get(1) {
        Some(raw_cents) if raw_cents.len() == 1 => raw_cents
            .parse::<i64>()
            .map(|value| value * 10)
            .map_err(|_| format!("Invalid money amount `{value}`"))?,
        Some(raw_cents) if raw_cents.len() == 2 => raw_cents
            .parse::<i64>()
            .map_err(|_| format!("Invalid money amount `{value}`"))?,
        Some(_) => return Err(format!("Invalid money amount `{value}`")),
        None => 0,
    };

    let total = dollars * 100 + cents;

    Ok(if is_negative { -total } else { total })
}

fn get_required_value<'a>(
    record: &'a csv::StringRecord,
    column: &str,
    column_index: &HashMap<String, usize>,
) -> Option<&'a str> {
    let index = column_index.get(&normalize_header(column))?;
    record.get(*index).filter(|value| !value.trim().is_empty())
}

fn get_optional_value<'a>(
    record: &'a csv::StringRecord,
    column: &str,
    column_index: &HashMap<String, usize>,
) -> Option<&'a str> {
    let index = column_index.get(&normalize_header(column))?;
    record
        .get(*index)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn missing_value_error(row: usize, column: &str) -> CsvImportError {
    CsvImportError {
        row: Some(row),
        column: Some(column.to_string()),
        message: format!("Missing value for `{column}`"),
    }
}

fn normalize_header(value: &str) -> String {
    value.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::transaction::TransactionDate;

    #[test]
    fn previews_csv_with_single_signed_amount_column() {
        let csv = include_str!("../../../examples/imports/signed-amount.csv");
        let preview = preview_transactions(
            csv,
            &CsvImportMapping {
                date_column: "Date".to_string(),
                date_format: DateFormat::IsoYearMonthDay,
                description_column: "Description".to_string(),
                amount_mapping: CsvAmountMapping::Single {
                    column: "Amount".to_string(),
                    convention: AmountSignConvention::NegativeIsOutflow,
                },
                account_column: Some("Account".to_string()),
                category_column: Some("Category".to_string()),
            },
        );

        assert_eq!(preview.errors, vec![]);
        assert_eq!(preview.transactions.len(), 2);
        assert_eq!(
            preview.transactions[0].amount,
            MoneyAmount::cad_cents(500_000)
        );
        assert_eq!(
            preview.transactions[1].amount,
            MoneyAmount::cad_cents(-320_050)
        );
        assert_eq!(
            preview.transactions[1].date,
            TransactionDate {
                year: 2026,
                month: 6,
                day: 2
            }
        );
        assert_eq!(
            preview.transactions[1].category,
            Some(TransactionCategory::Housing)
        );
    }

    #[test]
    fn previews_csv_with_split_debit_credit_columns() {
        let csv = include_str!("../../../examples/imports/split-debit-credit.csv");
        let preview = preview_transactions(
            csv,
            &CsvImportMapping {
                date_column: "Date".to_string(),
                date_format: DateFormat::MonthDayYear,
                description_column: "Description".to_string(),
                amount_mapping: CsvAmountMapping::Split {
                    debit_column: "Debit".to_string(),
                    credit_column: "Credit".to_string(),
                },
                account_column: None,
                category_column: None,
            },
        );

        assert_eq!(preview.errors, vec![]);
        assert_eq!(
            preview.transactions[0].amount,
            MoneyAmount::cad_cents(-12_345)
        );
        assert_eq!(
            preview.transactions[1].amount,
            MoneyAmount::cad_cents(1_000)
        );
    }

    #[test]
    fn reports_row_errors_without_crashing() {
        let csv = "Date,Description,Amount\n\
                   nope,Coffee,-4.25\n\
                   2026-06-02,,10.00\n";
        let preview = preview_transactions(
            csv,
            &CsvImportMapping {
                date_column: "Date".to_string(),
                date_format: DateFormat::IsoYearMonthDay,
                description_column: "Description".to_string(),
                amount_mapping: CsvAmountMapping::Single {
                    column: "Amount".to_string(),
                    convention: AmountSignConvention::NegativeIsOutflow,
                },
                account_column: None,
                category_column: None,
            },
        );

        assert_eq!(preview.transactions, vec![]);
        assert_eq!(preview.errors.len(), 2);
        assert_eq!(preview.errors[0].row, Some(2));
        assert_eq!(preview.errors[1].row, Some(3));
    }

    #[test]
    fn reports_missing_mapping_columns_before_parsing_rows() {
        let csv = "Date,Description,Amount\n2026-06-01,Coffee,-4.25\n";
        let preview = preview_transactions(
            csv,
            &CsvImportMapping {
                date_column: "Posted Date".to_string(),
                date_format: DateFormat::IsoYearMonthDay,
                description_column: "Description".to_string(),
                amount_mapping: CsvAmountMapping::Single {
                    column: "Amount".to_string(),
                    convention: AmountSignConvention::NegativeIsOutflow,
                },
                account_column: Some("Account".to_string()),
                category_column: None,
            },
        );

        assert_eq!(preview.transactions, vec![]);
        assert_eq!(preview.errors.len(), 2);
        assert_eq!(preview.errors[0].column, Some("Posted Date".to_string()));
        assert_eq!(preview.errors[1].column, Some("Account".to_string()));
    }

    #[test]
    fn parses_currency_formatting() {
        assert_eq!(parse_money_cents("$1,234.50"), Ok(123_450));
        assert_eq!(parse_money_cents("(12.34)"), Ok(-1_234));
        assert_eq!(parse_money_cents("12.3"), Ok(1_230));
        assert_eq!(parse_money_cents("CAD 12.34"), Ok(1_234));
    }
}
