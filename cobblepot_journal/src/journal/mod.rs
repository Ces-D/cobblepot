use chrono::{DateTime, Local};

pub enum AccountCategory {
    Asset,
    Liability,
    Expense,
    Revenue,
    Equity,
}

pub struct Account {
    name: String,
    description: String,
    opened: DateTime<Local>,
    closed: Option<DateTime<Local>>,
    category: AccountCategory,
}

impl Account {
    pub fn create(name: String, description: String, category: AccountCategory) -> Account {
        Account { name, description, category, opened: Local::now(), closed: None }
    }
}
