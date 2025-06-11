use std::{collections::HashSet, hash::Hash};

use crate::shared::{AccountType, RecurringStatus, ReportType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliOpenReport {
    pub report_tye: ReportType,
    pub id: Option<i32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub id: i32,
    pub name: String,
    pub entered_on: DateTime<Utc>,
    pub amount: f64,
}

impl Hash for Balance {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for Balance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Balance {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub assets: HashSet<Balance>,
    pub liabilities: HashSet<Balance>,
    pub total_assets: f64,
    pub total_liabilities: f64,
    pub net_position: f64,
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
    pub average: Option<f32>,
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

    pub recent: Option<Balance>,
    pub total_entries: i32,
    pub percent_delta: f32,
    pub timeline: ChangeTimeline,

    pub recurring: Vec<SimpleRecurringTransaction>,
}
