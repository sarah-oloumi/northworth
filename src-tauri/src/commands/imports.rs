use crate::imports::csv::{
    preview_transactions as preview_transactions_domain, CsvImportMapping, CsvImportPreview,
};

#[tauri::command]
pub fn preview_csv_transactions(csv_text: String, mapping: CsvImportMapping) -> CsvImportPreview {
    preview_transactions_domain(&csv_text, &mapping)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::transaction::DateFormat;
    use crate::imports::csv::{AmountSignConvention, CsvAmountMapping};

    #[test]
    fn previews_csv_transactions_through_command() {
        let preview = preview_csv_transactions(
            "Date,Description,Amount\n2026-01-01,Demo Grocery,-12.34\n".to_string(),
            CsvImportMapping {
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

        assert_eq!(preview.transactions.len(), 1);
        assert_eq!(preview.transactions[0].category, None);
        assert!(preview.errors.is_empty());
    }
}
