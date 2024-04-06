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
    created_on: chrono::NaiveDateTime,
    account_type: AccountType,
}

impl Account {
    pub fn new(name: String, account_type: AccountType) -> Account {
        Account {
            code: AccountCode(nanoid::nanoid!()),
            name,
            created_on: chrono::Local::now().naive_local(),
            account_type,
        }
    }

    pub fn account_code(self) -> AccountCode {
        self.code.clone()
    }
}
