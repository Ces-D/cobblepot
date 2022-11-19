use crate::cobblepot_core;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Result, Write};

#[derive(Serialize, Deserialize)]
struct AccountEntry {
    opened: DateTime<Utc>,
    closed: Option<DateTime<Utc>>,
    last_edit: DateTime<Utc>,
    account: Account,
}

#[derive(Serialize, Deserialize)]
pub enum AccountCategory {
    Asset,
    Liability,
    Expense,
    Revenue,
    Equity,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub company: Option<String>,
    pub name: String,
    pub description: String,
    pub category: AccountCategory,
}

#[derive(Serialize, Deserialize)]
pub struct ChartOfAccounts {
    entries: HashMap<String, AccountEntry>,
}

impl ChartOfAccounts {
    pub fn new() -> ChartOfAccounts {
        ChartOfAccounts::read_from_store_or_create()
    }

    pub fn read_from_store_or_create() -> ChartOfAccounts {
        let path = cobblepot_core::chart_of_accounts_path();
        let f = OpenOptions::new().read(true).open(path).expect("WTF");
        let reader = BufReader::new(f);
        match serde_json::from_reader(reader) {
            Ok(data) => data,
            Err(_) => ChartOfAccounts { entries: HashMap::new() },
        }
    }

    /// Opens a new entry in the Chart of Acconts
    /// If the account already existed, it updates the account information
    pub fn open_account(&mut self, entry: Account) -> Result<()> {
        let mut account_entry = AccountEntry {
            opened: chrono::Utc::now(),
            last_edit: chrono::Utc::now(),
            closed: None,
            account: entry,
        };

        let key = &account_entry.account.name;
        let entries = &mut self.entries;

        match entries.get(key) {
            Some(prev_entry) => {
                account_entry.opened = prev_entry.opened;
                account_entry.closed = prev_entry.closed;

                entries.insert(key.to_string(), account_entry);
                // Ok(account_entry)
                Ok(())
            },
            None => {
                entries.insert(key.to_string(), account_entry);
                // Ok(account_entry)
                Ok(())
            },
        }
    }

    /// Sets the clsoed status of an account. Officially closing it.
    pub fn close_account(&mut self, entry_name: String) -> Result<()> {
        match self.entries.get_mut(&entry_name) {
            Some(mut entry) => {
                if entry.closed.is_some() {
                    Ok(())
                } else {
                    entry.closed = Some(Utc::now());
                    Ok(())
                }
            },
            None => Ok(()),
        }
    }

    /// Once the session is complete, save everything to the Chart of Acounts Store. Changes are
    /// not permanent until this functin is called and successful.
    pub fn save(&self) -> Result<()> {
        let json_chart_of_accounts = serde_json::to_string_pretty(&self).expect("Failure to save");

        let path = cobblepot_core::chart_of_accounts_path();
        let f = OpenOptions::new().write(true).open(path).expect("WTF");
        let mut writer = BufWriter::new(f);
        match writer.write(json_chart_of_accounts.as_bytes()) {
            Ok(_) => writer.flush(),
            Err(_) => writer.flush(),
        }
    }

    /// Returns a vecotr of the entry names
    pub fn list_entries(&self) -> Vec<String> {
        self.entries.keys().cloned().collect::<Vec<String>>()
    }
}

// TODO: methods to add;
// account_meta - get the non account related info, opened, closed, last_edited
// TODO: edit the list_entries to filter by opened or closed or by date
