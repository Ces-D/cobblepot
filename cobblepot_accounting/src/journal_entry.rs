use core::fmt;
use std::str::FromStr;

use crate::account::AccountCode;
use cobblepot_core::error::CobblepotError;
use nanoid::nanoid;
use rusty_money::{iso::Currency, Money};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntryId(String);
impl fmt::Display for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for EntryId {
    type Err = CobblepotError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EntryId(s.to_string()))
    }
}

pub struct JournalEntry {
    id: EntryId,
    entered_on: chrono::NaiveDateTime,
    pub memo: String,
    amount: Money<'static, Currency>,
    account: AccountCode,
}

impl JournalEntry {
    pub fn new(
        memo: String,
        amount: Money<'static, Currency>,
        account: AccountCode,
    ) -> JournalEntry {
        JournalEntry {
            id: EntryId(nanoid!()),
            entered_on: chrono::Local::now().naive_local(),
            memo,
            amount,
            account,
        }
    }

    pub fn id(&self) -> EntryId {
        self.id.clone()
    }

    pub fn entered_on(&self) -> chrono::NaiveDateTime {
        self.entered_on
    }

    pub fn account_code(&self) -> AccountCode {
        self.account.clone()
    }

    pub fn amount(&self) -> Money<'static, Currency> {
        self.amount.clone()
    }
}
