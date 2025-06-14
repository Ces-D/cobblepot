use crate::schema::balance;
use chrono::{DateTime, NaiveDateTime, Utc};
use cli_docs_macro::CliDocs;
use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, CliDocs)]
pub struct JSONOpenBalance {
    #[cli_docs(description = "Memo for the balance entry")]
    pub memo: Option<String>,
    #[cli_docs(description = "Amount of the balance entry")]
    pub amount: f32,
    #[cli_docs(
        default = "UTC::now()",
        description = "Date and time the balance entry was created"
    )]
    pub entered_on: Option<DateTime<Utc>>,
    #[cli_docs(description = "ID of the account associated with the balance entry")]
    pub account_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, CliDocs)]
pub struct JSONUpdateBalance {
    #[cli_docs(description = "ID of the balance entry to update")]
    pub id: i32,
    #[cli_docs(description = "The new memo for the balance entry")]
    pub memo: Option<String>,
    #[cli_docs(description = "The new amount for the balance entry")]
    pub amount: Option<f32>,
    #[cli_docs(description = "The new date and time the balance entry was entered on")]
    pub entered_on: Option<DateTime<Utc>>,
    #[cli_docs(description = "The new ID of the account associated with the balance entry")]
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
