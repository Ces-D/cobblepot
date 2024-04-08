use crate::arguments::{account_code, amount, memo, parse_account_code, parse_amount, parse_memo};
use clap::{ArgMatches, Command};
use cobblepot_accounting::journal_entry::JournalEntry;
use cobblepot_core::error::CobblepotError;

pub fn create_command() -> Command {
    Command::new("add_journal_entry").about("Add a new entry to your finance journal").args([
        memo(),
        amount(),
        account_code(),
    ])
}

pub fn command_handler(matches: &ArgMatches) -> Result<(), CobblepotError> {
    let memo = parse_memo(matches)?;
    let amount = parse_amount(matches)?;
    let account_code = parse_account_code(matches)?;
    // TODO: Determine that the account code exists else return an error and prompt user to create this account
    // TODO: Add journal entry to the database
    // TODO: create a balance using the new entry
    let journal_entry = JournalEntry::new(memo.to_string(), amount, account_code);

    Ok(())
}
