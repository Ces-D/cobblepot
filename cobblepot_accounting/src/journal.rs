use crate::codes::{AccountCode, EntryCode};
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    account_code: AccountCode,
    entry_code: EntryCode,
    entered_on: chrono::NaiveDateTime,
    pub memo: String,
    pub transaction: Transaction,
}

impl Entry {
    pub fn new(memo: String, transaction: Transaction, account_code: AccountCode) -> Entry {
        Entry {
            entry_code: EntryCode::new(),
            account_code,
            entered_on: chrono::Local::now().naive_local(),
            memo,
            transaction,
        }
    }

    pub fn entry_code(&self) -> EntryCode {
        self.entry_code.clone()
    }

    pub fn account_code(&self) -> AccountCode {
        self.account_code.clone()
    }

    pub fn entered_on(&self) -> chrono::NaiveDateTime {
        self.entered_on
    }
}
