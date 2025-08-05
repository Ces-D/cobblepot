use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::responder::impl_json_responder;

/// Represents the JSON payload for applying a recurring transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONApplyRecurringTransaction {
    pub id: i32,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

/// Represents the result of applying a recurring transaction, including details about the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedRecurringTransaction {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub account_type: i32,

    pub update_balance_id: i32,
    pub applied_from: DateTime<Utc>,
    pub applied_to: DateTime<Utc>,
    pub applied_count: i32,
    pub amount: f32,
    pub total_applied_amount: f32,

    pub applied_on: Vec<DateTime<Utc>>,
}

impl_json_responder!(AppliedRecurringTransaction);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONApplyMarketInstrument {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFinancialMarketInstruments {
    pub applied_count: usize,
    pub new_balance_id: i32,
}

impl_json_responder!(AppliedFinancialMarketInstruments);
