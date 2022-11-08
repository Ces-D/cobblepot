use chrono::{DateTime, Local};
use std::fmt;
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
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let closed_date = match self.closed.ok_or(Local::now()) {
            Ok(date) => date.to_rfc2822(),
            Err(_) => String::from("NA"),
        };

        write!(
            f,
            "{}:{:>4}{:>8}{:>4}{:>6}",
            self.name,
            self.description,
            self.opened.to_rfc2822(),
            closed_date,
            self.category
        )
    }
}

//TODO: format this so that it print to new lines in the chart of accounts, this might require the
//write_all functions
