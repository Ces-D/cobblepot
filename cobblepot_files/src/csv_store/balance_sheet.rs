use crate::vault;
use cobblepot_accounting::account;
use cobblepot_accounting::balance_sheet;
use cobblepot_accounting::journal_entry;

pub struct BalanceSheet {
    config: vault::VaultConfig,
}

impl BalanceSheet {
    pub fn new(config: vault::VaultConfig) -> Self {
        BalanceSheet { config }
    }

    pub fn create_account_balance(balance: &balance_sheet::Balance) -> Option<()> {
        // check if file exists else create it
        // append the entry to the file
        // should return something of value - not sure yet
        todo!("Implement the create_account function")
    }

    pub fn find_most_recent_balance(account_code: account::AccountCode) {
        // find the most recent balance. should match the account code and be the most recent
        // created_on
        todo!("Implement the find_previous_balance function")
    }

    pub fn update_existing_balance(
        entry_id: journal_entry::EntryId,
        balance: balance_sheet::Balance,
    ) {
        // find the balance with the entry_id
        // check if the created_on is the most recent balance
        // if it is not, return an error prompting a new balance to be created
        //          reason: more recent balances were created using this incorrect balance, just
        //          intercept the chain of balances with a new correct balance
        // if it is, update the balance
        todo!("Implement the update_existing_balance function")
    }
}
