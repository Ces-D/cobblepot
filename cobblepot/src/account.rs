use chrono::{DateTime, Local};
use strum_macros::Display;

#[derive(Display)]
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

    pub fn to_csv_string(&self) -> String {
        let mut csv_string = String::new();
        csv_string.push_str(&self.name);
        csv_string.push(',');
        csv_string.push_str(&self.description);
        csv_string.push(',');
        csv_string.push_str(&self.opened.to_rfc3339());
        csv_string.push(',');
        if self.closed.is_some() {
            csv_string.push_str(&self.closed.expect("The if statement did nothing").to_rfc3339());
        };
        csv_string.push(',');
        csv_string.push_str(&self.category.to_string());

        csv_string
    }
}
