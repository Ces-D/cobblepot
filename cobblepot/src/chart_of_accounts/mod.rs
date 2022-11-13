use crate::core;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct ChartOfAccounts {
    entries: Vec<AccountEntry>,
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
    pub name: String,
    pub description: String,
    pub category: AccountCategory,
}

pub fn initialize_chart_of_accounts() {
    let path = core::chart_of_accounts_path();
    let chart_accounts_path = Path::new(path);
    let chart_of_accounts = ChartOfAccounts { entries: vec![] };
}
