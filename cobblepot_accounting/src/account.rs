use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::codes::AccountCode;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    code: AccountCode,
    name: String,
    pub description: String,
    created_on: chrono::DateTime<Local>,
    pub closed_on: Option<chrono::DateTime<Local>>,
}

impl Account {
    pub fn new(name: String, description: String) -> Account {
        Account {
            code: AccountCode::new(),
            name,
            description,
            created_on: Local::now(),
            closed_on: None,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn account_code(&self) -> AccountCode {
        self.code.clone()
    }
}
