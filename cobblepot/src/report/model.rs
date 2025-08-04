use crate::shared::{AccountType, RecurringStatus, ReportType};
use actix_web::{HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

/// Represents the JSON payload for requesting a new report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONOpenReport {
    pub report_type: ReportType,
    pub id: Option<i32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

/// Represents a balance for a specific account, used in reports.
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

/// A data unit returned from the database, representing an account balance with its type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAccountBalance {
    pub account_id: i32,
    pub balance_id: i32,
    pub name: String,
    pub entered_on: DateTime<Utc>,
    pub account_type: AccountType,
    pub amount: f32,
}

/// Assumption is that the balances have been organized by account type, and we can convert it to an AccountBalance
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

/// Represents a balance sheet report, summarizing assets and liabilities.
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

impl Responder for BalanceSheet {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}

/// A simplified representation of a recurring transaction for use in reports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleRecurringTransaction {
    pub id: i32,
    pub name: String,
    pub amount: f32,
    pub account_type: AccountType,
    pub apply_dates: Vec<DateTime<Utc>>,
    pub status: RecurringStatus,
}

/// A snapshot of the average change at a specific point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSnapShot {
    pub timeframe: DateTime<Utc>,
    pub average: f32,
}

/// Represents a timeline of changes, including multiple snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeTimeline {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub entry_count: i32,
    pub snapshots: Vec<ChangeSnapShot>,
}

/// A detailed, in-depth report for a single account.
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
    pub total_recurring_monthly_asset: f32,
    pub total_recurring_monthly_liability: f32,
}

impl Responder for AccountDeepDive {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}
