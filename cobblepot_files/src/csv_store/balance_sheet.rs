use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::io::BufWriter;

use cobblepot_accounting::transaction::TransactionVariant;

use crate::vault;

use super::utils::deserialize_csv;

pub struct BalanceSheet<'a> {
    config: &'a vault::VaultConfig,
}

impl<'a> BalanceSheet<'a> {
    pub fn new(config: &vault::VaultConfig) -> BalanceSheet<'_> {
        BalanceSheet { config }
    }

    fn open_location(&self, read: bool, write: bool) -> std::fs::File {
        let path = self.config.location_as_pathbuf().join("balance_sheet.csv");
        fs::OpenOptions::new()
            .read(read)
            .write(write)
            .create(true)
            .open(path)
            .expect("Could not open balance sheet file")
    }

    pub fn create_account_balance(
        &self,
        entry: &cobblepot_accounting::balance_sheet::Entry,
    ) -> Option<cobblepot_accounting::balance_sheet::Entry> {
        let writer = BufWriter::new(self.open_location(false, true));
        let mut writer = csv::Writer::from_writer(writer);
        match writer.serialize(entry) {
            Ok(_) => Some(entry.clone()),
            Err(e) => {
                eprintln!("Error writing to file: {}", e);
                return None;
            },
        }
    }

    /// Find a single accounts most recent balance
    pub fn find_most_recent_balance(
        &self,
        account_code: cobblepot_accounting::codes::AccountCode,
        account_variant: cobblepot_accounting::transaction::TransactionVariant,
    ) -> Option<cobblepot_accounting::balance_sheet::Entry> {
        let reader = BufReader::new(self.open_location(true, false));
        let balance_sheet =
            deserialize_csv::<cobblepot_accounting::balance_sheet::Entry>(reader).into_iter();
        let account_balances = balance_sheet.filter(|row| {
            row.account_code() == account_code && row.balance().variant() == account_variant
        });
        match account_balances.min_by(|x, y| x.cmp_by_created_on(y)) {
            Some(result) => Some(result),
            None => None,
        }
    }

    /// Find a list of accounts most recent balances
    pub fn find_most_recent_balances(
        &self,
        accounts: Vec<cobblepot_accounting::codes::AccountCode>,
    ) -> HashMap<
        cobblepot_accounting::codes::AccountCode,
        [Option<cobblepot_accounting::balance_sheet::Entry>; 2],
    > {
        let reader = BufReader::new(self.open_location(true, false));
        // sort the balances by created_at
        let mut balance_sheet =
            deserialize_csv::<cobblepot_accounting::balance_sheet::Entry>(reader);
        balance_sheet.sort_unstable_by(|a, b| a.cmp_by_created_on(b));
        let mut recent_balances: HashMap<
            cobblepot_accounting::codes::AccountCode,
            [Option<cobblepot_accounting::balance_sheet::Entry>; 2],
        > = HashMap::new();

        for account_code in accounts {
            // find the first instance of account code for each transaction
            let latest_asset = balance_sheet.clone().into_iter().find(|x| {
                x.account_code() == account_code
                    && x.balance().variant() == TransactionVariant::Asset
            });
            let latest_liability = balance_sheet.clone().into_iter().find(|x| {
                x.account_code() == account_code
                    && x.balance().variant() == TransactionVariant::Liability
            });
            recent_balances.insert(account_code, [latest_asset, latest_liability]);
        }

        recent_balances
    }
}
