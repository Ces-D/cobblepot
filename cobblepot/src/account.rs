use chrono::{DateTime, Local};

pub enum AccountCategory {
    Asset,
    Liability,
    Expense,
    Revenue,
    Equity,
}

pub struct Account {
    pub name: String,
    pub description: String,
    pub opened: DateTime<Local>,
    pub closed: Option<DateTime<Local>>,
    pub category: AccountCategory,
}

impl Account {
    pub fn create(name: String, description: String, category: AccountCategory) -> Account {
        Account { name, description, category, opened: Local::now(), closed: None }
    }
}
