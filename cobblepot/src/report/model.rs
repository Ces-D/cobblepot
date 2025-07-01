use crate::shared::{AccountType, RecurringStatus, ReportType};
use actix_web::{HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
pub struct JSONOpenReport {
    pub report_type: ReportType,
    pub id: Option<i32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
pub struct SimpleRecurringTransaction {
    pub id: i32,
    pub name: String,
    pub amount: f32,
    pub account_type: AccountType,
    pub apply_dates: Vec<DateTime<Utc>>,
    pub status: RecurringStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
pub struct ChangeSnapShot {
    pub timeframe: DateTime<Utc>,
    pub average: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
pub struct ChangeTimeline {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub entry_count: i32,
    pub snapshots: Vec<ChangeSnapShot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
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

impl Responder for AccountDeepDive {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}
