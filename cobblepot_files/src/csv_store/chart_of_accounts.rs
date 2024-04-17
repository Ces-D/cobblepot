use std::fs;
use std::io::{BufReader, BufWriter};

use crate::vault;
use cobblepot_accounting::account;

pub struct ChartOfAccounts {
    config: vault::VaultConfig,
}

impl ChartOfAccounts {
    pub fn new(config: vault::VaultConfig) -> Self {
        ChartOfAccounts { config }
    }

    fn open_location(&self, read: bool, write: bool) -> std::fs::File {
        let path = self.config.location_as_pathbuf().join("chart_of_accounts.csv");
        fs::OpenOptions::new()
            .read(read)
            .append(write)
            .create(true)
            .open(path)
            .expect("Unable to open the file")
    }

    pub fn create_account(&self, account: account::Account) -> Option<()> {
        let exists = self.find_account(account.account_code()).is_some();
        if exists {
            return None;
        }
        let writer = BufWriter::new(self.open_location(false, true));
        let mut writer = csv::Writer::from_writer(writer);
        match writer.serialize(account) {
            Ok(_) => Some(()),
            Err(e) => {
                eprintln!("Error writing to file: {}", e);
                return None;
            },
        }
    }

    // pub fn find_similar_accounts(account: account::AccountCode) -> Option<Vec<account::Account>> {
    //     // find all accounts that are similar in name and description to this account
    //     // determine how to do this
    //     todo!("Implement the find_similar_accounts function")
    // }

    pub fn find_account(&self, account_code: account::AccountCode) -> Option<account::Account> {
        let reader = BufReader::new(self.open_location(true, false));
        let mut reader = csv::Reader::from_reader(reader);
        let mut accounts = reader.deserialize::<account::Account>();
        let acct = accounts.find(|row| row.as_ref().unwrap().account_code() == account_code);
        if acct.is_some() {
            return Some(acct.unwrap().expect("Unable to deserialize account").clone());
        }
        return None;
    }

    // pub fn update_account(account: &account::Account) {
    //     // find the account with the account code
    //     // replace the account with the new account
    //     todo!("Implement the update_account function")
    // }
}
