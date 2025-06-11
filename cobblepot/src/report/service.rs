use crate::report::model::CliOpenReport;

pub fn create_balance_sheet_report(args: CliOpenReport, mut connection: diesel::SqliteConnection) {
    // TODO: Implement balance sheet report creation
}

pub fn create_deep_dive_account_report(
    args: CliOpenReport,
    mut connection: diesel::SqliteConnection,
) {
    // TODO: Implement deep dive account report creation
}
