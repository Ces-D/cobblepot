use core::fmt;
use std::str::FromStr;

use cobblepot_core::error::CobblepotError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct AccountCode(String);

impl AccountCode {
    pub fn new() -> AccountCode {
        AccountCode(nanoid::nanoid!())
    }
}

impl fmt::Display for AccountCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for AccountCode {
    type Err = CobblepotError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AccountCode(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntryCode(String);

impl EntryCode {
    pub fn new() -> EntryCode {
        EntryCode(nanoid::nanoid!())
    }
}

impl fmt::Display for EntryCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for EntryCode {
    type Err = CobblepotError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EntryCode(s.to_string()))
    }
}
