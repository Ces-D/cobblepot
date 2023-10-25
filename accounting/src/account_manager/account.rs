use chrono::{DateTime, Utc};

use super::account_code::AccountCode;

pub struct Account {
    pub name: String,
    pub description: String,
    code: AccountCode,
    opened: DateTime<Utc>,
    closed: Option<DateTime<Utc>>,
}

impl Account {
    pub fn new(name: String, description: String, code: AccountCode) -> Account {
        Account { name, description, code, opened: chrono::offset::Utc::now(), closed: None }
    }

    pub fn load_account(
        name: String,
        description: String,
        code: AccountCode,
        opened: DateTime<Utc>,
        closed: Option<DateTime<Utc>>,
    ) -> Account {
        Account { name, description, code, opened, closed }
    }

    pub fn get_code(&self) -> &AccountCode {
        &self.code
    }

    pub fn get_opened(&self) -> DateTime<Utc> {
        self.opened.to_owned()
    }

    pub fn close(&mut self) {
        if self.closed.is_some() {
            panic!("Account is already closed");
        }
        self.closed = Some(chrono::offset::Utc::now());
    }
}
