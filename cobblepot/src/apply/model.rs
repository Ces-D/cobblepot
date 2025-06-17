use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example=json!(JSONApplyRecurringTransaction{id:0, from:Utc::now(), to:None}))]
pub struct JSONApplyRecurringTransaction {
    pub id: i32,
    pub from: DateTime<Utc>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AppliedRecurringTransaction {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub account_type: i32,

    pub applied_from: DateTime<Utc>,
    pub applied_to: Option<DateTime<Utc>>,
    pub applied_count: i32,
    pub amount: f32,
    pub total_applied_amount: f32,

    pub applied_on: Vec<DateTime<Utc>>,
}
