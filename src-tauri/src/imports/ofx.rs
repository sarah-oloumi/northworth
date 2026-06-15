use crate::domain::money::MoneyAmount;
use crate::domain::transaction::{ImportReviewStatus, TransactionDate, TransactionRecord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfxImportPreview {
    pub transactions: Vec<TransactionRecord>,
    pub errors: Vec<OfxImportError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfxImportError {
    pub transaction_index: Option<usize>,
    pub field: Option<String>,
    pub message: String,
}

pub fn preview_transactions(ofx_contents: &str) -> OfxImportPreview {
    let blocks = tagged_blocks(ofx_contents, "STMTTRN");

    if blocks.is_empty() {
        return OfxImportPreview {
            transactions: vec![],
            errors: vec![OfxImportError {
                transaction_index: None,
                field: None,
                message: "No OFX/QFX transactions found".to_string(),
            }],
        };
    }

    let mut transactions = Vec::new();
    let mut errors = Vec::new();

    for (index, block) in blocks.iter().enumerate() {
        match parse_transaction(block, index + 1) {
            Ok(transaction) => transactions.push(transaction),
            Err(mut transaction_errors) => errors.append(&mut transaction_errors),
        }
    }

    OfxImportPreview {
        transactions,
        errors,
    }
}

fn parse_transaction(
    block: &str,
    transaction_index: usize,
) -> Result<TransactionRecord, Vec<OfxImportError>> {
    let mut errors = Vec::new();
    let date = match tag_value(block, "DTPOSTED").and_then(parse_ofx_date) {
        Some(date) => Some(date),
        None => {
            errors.push(error(
                transaction_index,
                "DTPOSTED",
                "Missing or invalid posted date",
            ));
            None
        }
    };
    let amount = match tag_value(block, "TRNAMT").and_then(parse_ofx_amount) {
        Some(amount) => Some(amount),
        None => {
            errors.push(error(
                transaction_index,
                "TRNAMT",
                "Missing or invalid transaction amount",
            ));
            None
        }
    };
    let description = tag_value(block, "NAME")
        .or_else(|| tag_value(block, "MEMO"))
        .filter(|value| !value.trim().is_empty());

    if description.is_none() {
        errors.push(error(
            transaction_index,
            "NAME",
            "Missing transaction description",
        ));
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(TransactionRecord {
        source_row: transaction_index,
        date: date.expect("date checked before transaction construction"),
        description: description
            .expect("description checked before transaction construction")
            .to_string(),
        amount: amount.expect("amount checked before transaction construction"),
        account_id: None,
        account_name: None,
        category: None,
        import_status: ImportReviewStatus::PendingReview,
    })
}

fn tagged_blocks<'a>(contents: &'a str, tag: &str) -> Vec<&'a str> {
    let uppercase = contents.to_uppercase();
    let open_tag = format!("<{tag}>");
    let close_tag = format!("</{tag}>");
    let mut blocks = Vec::new();
    let mut search_from = 0;

    while let Some(open_offset) = uppercase[search_from..].find(&open_tag) {
        let block_start = search_from + open_offset + open_tag.len();
        let Some(close_offset) = uppercase[block_start..].find(&close_tag) else {
            break;
        };
        let block_end = block_start + close_offset;
        blocks.push(&contents[block_start..block_end]);
        search_from = block_end + close_tag.len();
    }

    blocks
}

fn tag_value<'a>(contents: &'a str, tag: &str) -> Option<&'a str> {
    let uppercase = contents.to_uppercase();
    let open_tag = format!("<{tag}>");
    let start = uppercase.find(&open_tag)? + open_tag.len();
    let rest = &contents[start..];
    let end = rest.find('<').unwrap_or(rest.len());
    let value = rest[..end].trim();

    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn parse_ofx_date(value: &str) -> Option<TransactionDate> {
    let digits = value
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>();

    if digits.len() < 8 {
        return None;
    }

    let year = digits[0..4].parse::<u16>().ok()?;
    let month = digits[4..6].parse::<u8>().ok()?;
    let day = digits[6..8].parse::<u8>().ok()?;

    TransactionDate::new(year, month, day).ok()
}

fn parse_ofx_amount(value: &str) -> Option<MoneyAmount> {
    let cleaned = value.trim().replace(['$', ','], "");
    let amount = cleaned.parse::<f64>().ok()?;

    Some(MoneyAmount::cad_cents((amount * 100.0).round() as i64))
}

fn error(transaction_index: usize, field: &str, message: &str) -> OfxImportError {
    OfxImportError {
        transaction_index: Some(transaction_index),
        field: Some(field.to_string()),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previews_synthetic_ofx_transactions() {
        let preview = preview_transactions(include_str!("../../../examples/imports/sample.ofx"));

        assert_eq!(preview.errors, vec![]);
        assert_eq!(preview.transactions.len(), 2);
        assert_eq!(preview.transactions[0].description, "Demo paycheque");
        assert_eq!(
            preview.transactions[0].amount,
            MoneyAmount::cad_cents(500_000)
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
            preview.transactions[1].amount,
            MoneyAmount::cad_cents(-12_480)
        );
        assert_eq!(preview.transactions[1].account_name, None);
    }

    #[test]
    fn previews_synthetic_qfx_transactions() {
        let preview = preview_transactions(include_str!("../../../examples/imports/sample.qfx"));

        assert_eq!(preview.errors, vec![]);
        assert_eq!(preview.transactions.len(), 1);
        assert_eq!(preview.transactions[0].description, "Demo fuel");
        assert_eq!(
            preview.transactions[0].amount,
            MoneyAmount::cad_cents(-6_425)
        );
    }

    #[test]
    fn reports_missing_transactions() {
        let preview = preview_transactions("<OFX></OFX>");

        assert_eq!(preview.transactions, vec![]);
        assert_eq!(preview.errors.len(), 1);
        assert_eq!(preview.errors[0].transaction_index, None);
    }

    #[test]
    fn reports_transaction_field_errors_without_raw_values() {
        let preview =
            preview_transactions("<OFX><STMTTRN><DTPOSTED>bad<TRNAMT>wat<NAME></STMTTRN></OFX>");

        assert_eq!(preview.transactions, vec![]);
        assert_eq!(preview.errors.len(), 3);
        assert!(preview
            .errors
            .iter()
            .all(|error| !error.message.contains("bad") && !error.message.contains("wat")));
    }
}
