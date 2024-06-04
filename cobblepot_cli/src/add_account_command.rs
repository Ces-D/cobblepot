use crate::arguments::{
    account_description, account_name, parse_account_description, parse_account_name,
};
use clap::{ArgMatches, Command};
use cobblepot_accounting::account::Account;
use cobblepot_core::error::CobblepotError;
use cobblepot_files::csv_store::chart_of_accounts::ChartOfAccounts;
use cobblepot_files::vault::VaultConfig;

pub fn create_command() -> Command {
    Command::new("add_account")
        .about("Add a new account to your chart of accounts")
        .args([account_name(), account_description()])
}

pub fn command_handler(
    matches: &ArgMatches,
    vault_config: &VaultConfig,
) -> Result<(), CobblepotError> {
    let account_name = parse_account_name(matches)?;
    let account_description = parse_account_description(matches)?;

    let chart_of_accounts = ChartOfAccounts::new(vault_config);
    match chart_of_accounts
        .create_account(Account::new(account_name.clone(), account_description.clone()))
    {
        Some(_) => return Ok(()),
        None => Err(CobblepotError::AddChartOfAccountsCliError(
            "Error unable to create chart of accounts account",
        )),
    }
}
