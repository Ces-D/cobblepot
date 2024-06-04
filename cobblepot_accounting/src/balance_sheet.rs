use cobblepot_core::error::CobblepotError;
use serde::{Deserialize, Serialize};

use crate::codes::{AccountCode, EntryCode};
use crate::transaction::Transaction;

/// Balance sheet entries are records that summarize the state of an account type
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    entry_code: EntryCode,
    account_code: AccountCode,
    balance: Transaction,
    updated_on: Option<chrono::NaiveDateTime>,
    created_on: chrono::NaiveDateTime,
}

impl Entry {
    pub fn new(entry_code: EntryCode, account_code: AccountCode, balance: Transaction) -> Self {
        Self {
            entry_code,
            account_code,
            balance,
            updated_on: None,
            created_on: chrono::Local::now().naive_local(),
        }
    }

    pub fn account_code(&self) -> AccountCode {
        self.account_code.clone()
    }

    pub fn entry_code(&self) -> EntryCode {
        self.entry_code.clone()
    }

    pub fn balance(&self) -> Transaction {
        self.balance.clone()
    }

    pub fn created_on(&self) -> chrono::NaiveDateTime {
        self.created_on
    }

    pub fn update_balance_by(&mut self, transaction: Transaction) -> Result<(), CobblepotError> {
        self.balance = self.balance.add(transaction)?;
        self.updated_on = Some(chrono::Local::now().naive_local());
        Ok(())
    }

    pub fn cmp_by_created_on(&self, other: &Entry) -> std::cmp::Ordering {
        self.created_on().cmp(&other.created_on())
    }
}
