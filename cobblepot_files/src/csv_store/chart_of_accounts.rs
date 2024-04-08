use crate::vault;
use cobblepot_accounting::account;

pub struct ChartOfAccounts {
    config: vault::VaultConfig,
}

impl ChartOfAccounts {
    pub fn new(config: vault::VaultConfig) -> Self {
        ChartOfAccounts { config }
    }

    pub fn create_account(account: account::Account) -> Option<()> {
        // check if file exists else create it
        // append the account to the file
        // should return something of value - not sure yet
        // important that this succeeds before any other operation
        todo!("Implement the create_account function")
    }

    pub fn find_similar_accounts(account: account::AccountCode) -> Option<Vec<account::Account>> {
        // find all accounts that are similar in name and description to this account
        // determine how to do this
        todo!("Implement the find_similar_accounts function")
    }

    pub fn find_account(account: account::AccountCode) -> Option<account::Account> {
        // find the account with the account code
        todo!("Implement the find_account function")
    }

    pub fn update_account(account: &account::Account) {
        // find the account with the account code
        // replace the account with the new account
        todo!("Implement the update_account function")
    }
}
