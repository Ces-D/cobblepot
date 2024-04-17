use serde::{Deserialize, Serialize};

use crate::account::AccountCode;
use crate::journal_entry::EntryId;
use crate::money::Money;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Balance {
    /// PK is id of entry that calculated
    entry: EntryId,
    account: AccountCode,
    balance: Money,
    updated_on: Option<chrono::NaiveDateTime>,
    created_on: chrono::NaiveDateTime,
}

impl Balance {
    pub fn new(entry: EntryId, account: AccountCode, balance: Option<Money>) -> Balance {
        Balance {
            entry,
            account,
            balance: balance.unwrap_or(Money::from_major(0)),
            updated_on: None,
            created_on: chrono::Local::now().naive_local(),
        }
    }

    pub fn account_code(&self) -> AccountCode {
        self.account.clone()
    }

    pub fn entry_id(&self) -> EntryId {
        self.entry.clone()
    }

    pub fn created_on(&self) -> chrono::NaiveDateTime {
        self.created_on
    }

    pub fn update_balance(&mut self, balance: Money) {
        self.balance = balance;
        self.updated_on = Some(chrono::Local::now().naive_local());
    }

    pub fn update_balance_by_amount(&mut self, amount: Money) {
        self.balance = self.balance.clone() + amount;
        self.updated_on = Some(chrono::Local::now().naive_local());
    }
}
