use chrono::{DateTime, Local};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use cobblepot_core::error::CobblepotError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Should not be called directly, use `Account::new` instead.
pub struct AccountCode(String);
impl fmt::Display for AccountCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}
impl FromStr for AccountCode {
    type Err = CobblepotError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AccountCode(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    code: AccountCode,
    name: String,
    pub description: String,
    created_on: chrono::DateTime<Local>,
    closed_on: Option<chrono::DateTime<Local>>,
    account_type: AccountType,
}

impl Account {
    pub fn new(name: String, description: String, account_type: AccountType) -> Account {
        Account {
            code: AccountCode(nanoid::nanoid!()),
            name,
            description,
            created_on: Local::now(),
            closed_on: None,
            account_type,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn account_code(&self) -> AccountCode {
        self.code.clone()
    }

    pub fn closed_on(&self) -> Option<DateTime<Local>> {
        self.closed_on.clone()
    }

    pub fn close_account(&mut self) {
        if self.closed_on.is_none() {
            self.closed_on = Some(Local::now())
        }
    }
}
