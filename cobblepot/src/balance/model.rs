use crate::schema::balance;
use actix_web::{HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
pub struct JSONListBalances {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub entered_after: Option<DateTime<Utc>>,
    pub account_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
pub struct JSONOpenBalance {
    pub memo: Option<String>,
    pub amount: f32,
    pub entered_on: Option<DateTime<Utc>>,
    pub account_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "cobblepot_types.ts")]
pub struct JSONUpdateBalance {
    pub id: i32,
    pub memo: Option<String>,
    pub amount: Option<f32>,
    pub entered_on: Option<DateTime<Utc>>,
    pub account_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=balance)]
pub struct InsertableBalance {
    pub memo: String,
    pub amount: f32,
    pub entered_on: NaiveDateTime,
    pub account_id: i32,
}

impl From<JSONOpenBalance> for InsertableBalance {
    fn from(value: JSONOpenBalance) -> Self {
        Self {
            memo: value
                .memo
                .unwrap_or(format!("Updating financial record - {}", Utc::now().to_rfc2822())),
            amount: value.amount,
            entered_on: value.entered_on.unwrap_or(Utc::now()).naive_utc(),
            account_id: value.account_id,
        }
    }
}

#[derive(Debug, AsChangeset, Identifiable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=balance)]
pub struct UpdatableBalance {
    pub id: i32,
    pub memo: Option<String>,
    pub amount: Option<f32>,
    pub entered_on: Option<NaiveDateTime>,
    pub account_id: Option<i32>,
}

impl From<JSONUpdateBalance> for UpdatableBalance {
    fn from(value: JSONUpdateBalance) -> Self {
        Self {
            id: value.id,
            memo: value.memo,
            amount: value.amount,
            entered_on: value.entered_on.map(|v| v.naive_utc()),
            account_id: value.account_id,
        }
    }
}

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize, TS)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=balance)]
#[ts(export, export_to = "cobblepot_types.ts")]
pub struct Balance {
    pub id: i32,
    pub memo: String,
    pub amount: f32,
    pub entered_on: NaiveDateTime,
    pub account_id: i32,
}

impl Responder for Balance {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}

#[derive(Debug, Serialize)]
pub struct BalanceList(pub Vec<Balance>);

impl Responder for BalanceList {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self.0).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::iter::repeat_with;

    use chrono::{Months, Utc};

    use crate::balance::model::{Balance, JSONUpdateBalance};

    /// Changes all but the `id` and `account_id` fields of the original balance
    pub fn create_dummy_update_balance(balance: &Balance) -> JSONUpdateBalance {
        JSONUpdateBalance {
            id: balance.id,
            account_id: None,
            memo: Some(repeat_with(fastrand::alphanumeric).take(10).collect()),
            amount: Some(balance.amount + balance.amount),
            entered_on: Some(Utc::now().checked_sub_months(Months::new(6)).unwrap()),
        }
    }
}
