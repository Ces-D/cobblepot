use crate::schema::account;
use crate::shared::{AccountType, DATETIME_FORMAT};
use chrono::{DateTime, Utc, serde::ts_milliseconds_option};
use diesel::prelude::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CliGetAccount {
    pub id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CliOpenAccount {
    pub name: String,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub account_type: Option<AccountType>,
    #[serde(with = "ts_milliseconds_option")]
    pub opened_on: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub closed_on: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CliUpdateAccount {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub account_type: Option<AccountType>,
    #[serde(with = "ts_milliseconds_option")]
    pub opened_on: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub closed_on: Option<DateTime<Utc>>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CliCloseAccount {
    pub id: i32,
    #[serde(with = "ts_milliseconds_option")]
    pub closed_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=account)]
pub struct InsertableAccount {
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub account_type: i32,
    pub opened_on: String,
    pub closed_on: Option<String>,
}

impl From<CliOpenAccount> for InsertableAccount {
    fn from(value: CliOpenAccount) -> Self {
        Self {
            name: value.name,
            description: value.description,
            owner: value.owner.unwrap_or("me".to_string()),
            account_type: value.account_type.unwrap_or(AccountType::Asset) as i32,
            opened_on: value.opened_on.unwrap_or(Utc::now()).format(DATETIME_FORMAT).to_string(),
            closed_on: value.closed_on.map(|v| v.format(DATETIME_FORMAT).to_string()),
        }
    }
}

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=account)]
pub struct UpdatableAccount {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub account_type: Option<i32>,
    pub opened_on: Option<String>,
    pub closed_on: Option<String>,
}

impl From<CliUpdateAccount> for UpdatableAccount {
    fn from(value: CliUpdateAccount) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            owner: value.owner,
            account_type: value.account_type.map(|v| v as i32),
            opened_on: value.opened_on.map(|v| v.format(DATETIME_FORMAT).to_string()),
            closed_on: value.closed_on.map(|v| v.format(DATETIME_FORMAT).to_string()),
        }
    }
}

#[derive(Debug, AsChangeset, Identifiable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=account)]
pub struct ClosableAccount {
    pub id: i32,
    pub closed_on: Option<String>,
}

#[derive(Debug, Queryable, Identifiable, Serialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=account)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub account_type: i32,
    pub opened_on: String,
    pub closed_on: Option<String>,
}
