use crate::shared::{AccountType, RecurringStatus, ReportType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliOpenReport {
    pub report_tye: ReportType,
    pub id: Option<i32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub account_id: i32,
    pub balance_id: i32,
    pub name: String,
    pub entered_on: DateTime<Utc>,
    pub amount: f32,
}

impl Hash for AccountBalance {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.account_id.hash(state);
    }
}
impl PartialEq for AccountBalance {
    fn eq(&self, other: &Self) -> bool {
        self.account_id == other.account_id
    }
}
impl Eq for AccountBalance {}

/// A data unit returned from the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAccountBalance {
    pub account_id: i32,
    pub balance_id: i32,
    pub name: String,
    pub entered_on: DateTime<Utc>,
    pub account_type: AccountType,
    pub amount: f32,
}

/// Assumption is that the account type has already been validated, and we can convert it to an AccountBalance
impl From<&LoadAccountBalance> for AccountBalance {
    fn from(load_balance: &LoadAccountBalance) -> Self {
        AccountBalance {
            account_id: load_balance.account_id,
            balance_id: load_balance.balance_id,
            name: load_balance.name.clone(),
            entered_on: load_balance.entered_on,
            amount: load_balance.amount,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub assets: HashSet<AccountBalance>,
    pub liabilities: HashSet<AccountBalance>,
    pub assets_total: f32,
    pub liabilities_total: f32,
    pub net_position: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleRecurringTransaction {
    pub id: i32,
    pub name: String,
    pub amount: f32,
    pub account_type: AccountType,
    pub status: RecurringStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSnapShot {
    pub timeframe: DateTime<Utc>,
    pub average: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeTimeline {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub entry_count: i32,
    pub snapshots: Vec<ChangeSnapShot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeepDive {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub account_type: AccountType,
    pub opened_on: DateTime<Utc>,
    pub closed_on: Option<DateTime<Utc>>,

    pub recent: Option<AccountBalance>,
    pub total_entries: usize,

    pub timeline: ChangeTimeline,
    pub recurring: Vec<SimpleRecurringTransaction>,
}
