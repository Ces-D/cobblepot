use core::fmt;
use std::str::FromStr;

use cobblepot_core::error::CobblepotError;

pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

pub struct Account {
    code: AccountCode,
    name: String,
    pub description: String,
    created_on: chrono::NaiveDateTime,
    closed_on: Option<chrono::NaiveDateTime>,
    account_type: AccountType,
}

impl Account {
    pub fn new(name: String, description: String, account_type: AccountType) -> Account {
        Account {
            code: AccountCode(nanoid::nanoid!()),
            name,
            description,
            created_on: chrono::Local::now().naive_local(),
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

    pub fn closed_on(&self) -> Option<chrono::NaiveDateTime> {
        self.closed_on.clone()
    }

    pub fn close_account(&mut self) {
        if self.closed_on.is_none() {
            self.closed_on = Some(chrono::Local::now().naive_local())
        }
    }
}
