use crate::arguments::{account_code, amount, memo, parse_account_code, parse_amount, parse_memo};
use clap::{ArgMatches, Command};
use cobblepot_accounting::balance_sheet::Balance;
use cobblepot_accounting::journal_entry::JournalEntry;
use cobblepot_core::error::CobblepotError;
use cobblepot_files::csv_store::balance_sheet::BalanceSheet;
use cobblepot_files::csv_store::chart_of_accounts::ChartOfAccounts;
use cobblepot_files::csv_store::journal::Journal;
use cobblepot_files::vault::VaultConfig;

pub fn create_command() -> Command {
    Command::new("add_journal_entry").about("Add a new entry to your finance journal").args([
        memo(),
        amount(),
        account_code(),
    ])
}

pub fn command_handler(
    matches: &ArgMatches,
    vault_config: &VaultConfig,
) -> Result<(), CobblepotError> {
    let memo = parse_memo(matches)?;
    let amount = parse_amount(matches)?;
    let account_code = parse_account_code(matches)?;

    let chart_of_accounts = ChartOfAccounts::new(vault_config);
    let account_exists = chart_of_accounts.find_account(account_code.clone()).is_some();

    if account_exists {
        let journal_entry = JournalEntry::new(memo.to_string(), amount, account_code);
        let journal = Journal::new(vault_config);
        match journal.create_entry(&journal_entry) {
            Some(entry) => {
                let mut account_balance =
                    Balance::new(entry.entry_id(), entry.account_code(), None);
                let balance_sheet = BalanceSheet::new(vault_config);
                let last_balance = balance_sheet.find_most_recent_balance(entry.account_code());

                if last_balance.is_some() {
                    account_balance.update_balance_by_amount(last_balance.unwrap().balance());
                }
                match balance_sheet.create_account_balance(&account_balance) {
                    Some(_) => Ok(()),
                    None => Err(CobblepotError::AddJournalEntryCliError(
                        "Error creating journal entries balance",
                    )),
                }
            },
            None => Err(CobblepotError::AddJournalEntryCliError("Error creating journal entry")),
        }
    } else {
        return Err(CobblepotError::AddJournalEntryCliError("Account does not exist"));
    }
}
