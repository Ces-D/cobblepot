use clap::{ArgMatches, Command};
use cobblepot_core::error::CobblepotError;
use cobblepot_files::csv_store::balance_sheet::BalanceSheet;
use cobblepot_files::csv_store::chart_of_accounts::ChartOfAccounts;
use cobblepot_files::vault::VaultConfig;
use serde_json;

pub fn create_command() -> Command {
    Command::new("get_balances").about("Get all the most current balances for your accounts")
}

pub fn command_handler(
    _matches: &ArgMatches,
    vault_config: &VaultConfig,
) -> Result<(), CobblepotError> {
    let balance_sheet = BalanceSheet::new(vault_config);
    let chart_of_accounts = ChartOfAccounts::new(vault_config);
    let accounts =
        chart_of_accounts.list_accounts().into_iter().map(|el| el.account_code()).collect();
    let recent_balances = balance_sheet.find_most_recent_balances(accounts);
    let content = serde_json::to_string_pretty(&recent_balances).unwrap();
    print!("{}", content);
    Ok(())
}
