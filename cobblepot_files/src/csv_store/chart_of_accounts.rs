use std::fs;
use std::io::{BufReader, BufWriter};

use crate::vault;

use super::utils::deserialize_csv;

pub struct ChartOfAccounts<'a> {
    config: &'a vault::VaultConfig,
}

impl<'a> ChartOfAccounts<'a> {
    pub fn new(config: &vault::VaultConfig) -> ChartOfAccounts<'_> {
        ChartOfAccounts { config }
    }

    fn open_location(&self, read: bool, write: bool) -> std::fs::File {
        let path = self.config.location_as_pathbuf().join("chart_of_accounts.csv");
        if !path.exists() {
            match fs::OpenOptions::new().write(true).create_new(true).open(path) {
                Ok(_) => self.open_location(read, write),
                Err(_) => panic!("Unable to create the file"),
            }
        } else {
            fs::OpenOptions::new()
                .read(read)
                .append(write)
                .open(path)
                .expect("Unable to open the file")
        }
    }

    pub fn create_account(&self, account: cobblepot_accounting::account::Account) -> Option<()> {
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

    pub fn find_account(
        &self,
        account_code: cobblepot_accounting::codes::AccountCode,
    ) -> Option<cobblepot_accounting::account::Account> {
        let reader = BufReader::new(self.open_location(true, false));

        let mut accounts =
            deserialize_csv::<cobblepot_accounting::account::Account>(reader).into_iter();
        let acct = accounts.find(|row| row.account_code() == account_code);
        if acct.is_some() {
            return Some(acct.clone())?;
        }
        return None;
    }

    pub fn list_accounts(&self) -> Vec<cobblepot_accounting::account::Account> {
        let reader = BufReader::new(self.open_location(true, false));
        deserialize_csv::<cobblepot_accounting::account::Account>(reader)
    }

    // pub fn update_account(account: &account::Account) {
    //     // find the account with the account code
    //     // replace the account with the new account
    //     todo!("Implement the update_account function")
    // }
}
