pub mod commands;
pub mod domain;
pub mod imports;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::budget::calculate_budget_month,
            commands::budget::summarize_budget_pies,
            commands::imports::preview_csv_transactions,
            commands::imports::preview_ofx_transactions,
            commands::market_data::assess_market_data_freshness,
            commands::market_data::list_market_data_refresh_policies,
            commands::reports::build_calendar,
            commands::reports::build_cash_flow,
            commands::reports::build_net_worth,
            commands::reports::build_spending_analysis,
            commands::reports::build_transaction_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Northworth");
}
