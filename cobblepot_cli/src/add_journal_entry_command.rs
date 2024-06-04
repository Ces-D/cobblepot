use crate::arguments::{
    account_code, amount, currency, memo, parse_account_code, parse_amount, parse_currency,
    parse_memo,
};
use crate::arguments::{parse_transaction_variant, transaction_variant};
use clap::{ArgMatches, Command};
use cobblepot_accounting::balance_sheet;
use cobblepot_accounting::journal;
use cobblepot_accounting::transaction::Transaction;
use cobblepot_core::error::CobblepotError;
use cobblepot_files::csv_store::balance_sheet::BalanceSheet;
use cobblepot_files::csv_store::chart_of_accounts::ChartOfAccounts;
use cobblepot_files::csv_store::journal::Journal;
use cobblepot_files::vault::VaultConfig;

pub fn create_command() -> Command {
    Command::new("add_journal_entry").about("Add a new entry to your finance journal").args([
        memo(),
        currency(),
        amount(),
        transaction_variant(),
        account_code(),
    ])
}

pub fn command_handler(
    matches: &ArgMatches,
    vault_config: &VaultConfig,
) -> Result<(), CobblepotError> {
    let memo = parse_memo(matches)?;
    let currency = parse_currency(matches)?;
    let amount = parse_amount(matches, currency)?;
    let transaction_type = parse_transaction_variant(matches)?;
    let account_code = parse_account_code(matches)?;

    let transaction = Transaction::new(transaction_type, amount);
    let chart_of_accounts = ChartOfAccounts::new(vault_config);
    let account_exists = chart_of_accounts.find_account(account_code.clone()).is_some();

    if account_exists {
        let journal_entry =
            journal::Entry::new(memo.to_string(), transaction.clone(), account_code);
        let journal = Journal::new(vault_config);
        match journal.create_entry(&journal_entry) {
            Some(entry) => {
                let mut account_balance = balance_sheet::Entry::new(
                    entry.entry_code(),
                    entry.account_code(),
                    transaction,
                );
                let balance_sheet = BalanceSheet::new(vault_config);
                let last_balance = balance_sheet
                    .find_most_recent_balance(entry.account_code(), entry.transaction.variant());

                if last_balance.is_some() {
                    account_balance.update_balance_by(last_balance.unwrap().balance())?;
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
