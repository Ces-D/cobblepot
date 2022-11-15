use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChartOfAccounts {
    pub entries: Vec<AccountEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct AccountEntry {
    pub opened: DateTime<Utc>,
    pub closed: Option<DateTime<Utc>>,
    pub account: Account,
}

#[derive(Serialize, Deserialize)]
pub enum AccountCategory {
    Asset,
    Liability,
    Expense,
    Revenue,
    Equity,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub company: Option<String>,
    pub name: String,
    pub description: String,
    pub category: AccountCategory,
}
