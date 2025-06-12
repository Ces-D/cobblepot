use crate::schema::balance;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliOpenBalance {
    pub memo: Option<String>,
    pub amount: f32,
    pub entered_on: Option<DateTime<Utc>>,
    pub account_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliUpdateBalance {
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

impl From<CliOpenBalance> for InsertableBalance {
    fn from(value: CliOpenBalance) -> Self {
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

impl From<CliUpdateBalance> for UpdatableBalance {
    fn from(value: CliUpdateBalance) -> Self {
        Self {
            id: value.id,
            memo: value.memo,
            amount: value.amount,
            entered_on: value.entered_on.map(|v| v.naive_utc()),
            account_id: value.account_id,
        }
    }
}

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=balance)]
pub struct Balance {
    pub id: i32,
    pub memo: String,
    pub amount: f32,
    pub entered_on: NaiveDateTime,
    pub account_id: i32,
}
