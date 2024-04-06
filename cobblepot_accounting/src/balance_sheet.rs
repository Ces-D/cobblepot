use rusty_money::iso;
use rusty_money::{iso::Currency, Money};

use crate::account::AccountCode;
use crate::journal_entry::EntryId;

pub struct Balance {
    /// PK is id of entry that calculated
    entry: EntryId,
    account: AccountCode,
    balance: Money<'static, Currency>,
    updated_on: chrono::NaiveDateTime,
    created_on: chrono::NaiveDateTime,
}

impl Balance {
    pub fn new(
        entry: EntryId,
        account: AccountCode,
        balance: Option<Money<'static, Currency>>,
        updated_on: Option<chrono::NaiveDateTime>,
    ) -> Balance {
        Balance {
            entry,
            account,
            balance: balance.unwrap_or(Money::from_major(0, iso::USD)),
            updated_on: updated_on.unwrap_or(chrono::Local::now().naive_local()),
            created_on: chrono::Local::now().naive_local(),
        }
    }

    pub fn account_code(&self) -> AccountCode {
        self.account.clone()
    }

    pub fn entry_id(&self) -> EntryId {
        self.entry.clone()
    }

    pub fn update_balance_sheet(&mut self, amount: Money<'static, Currency>) {
        self.balance = self.balance.clone() + amount;
        self.updated_on = chrono::Local::now().naive_local();
    }
}
