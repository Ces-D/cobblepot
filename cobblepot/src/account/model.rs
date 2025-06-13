use crate::{schema::account, shared::AccountType};
use chrono::{DateTime, NaiveDateTime, Utc};
use cli_docs_macro::CliDocs;
use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CliGetAccount {
    pub id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, CliDocs)]
pub struct CliOpenAccount {
    #[cli_docs(description = "The name of the account")]
    pub name: String,
    #[cli_docs(description = "The description of the account")]
    pub description: Option<String>,
    #[cli_docs(description = "The owner of the account")]
    pub owner: Option<String>,
    #[cli_docs(default = "0", description = "The type of the account. Asset=0, Liability=1")]
    pub account_type: Option<AccountType>,
    #[cli_docs(default = "Utc::now()", description = "The date and time the account was opened")]
    pub opened_on: Option<DateTime<Utc>>,
    #[cli_docs(description = "The date and time the account was closed")]
    pub closed_on: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, CliDocs)]
pub struct CliUpdateAccount {
    #[cli_docs(description = "The ID of the account")]
    pub id: i32,
    #[cli_docs(description = "The new name of the account")]
    pub name: Option<String>,
    #[cli_docs(description = "The new description of the account")]
    pub description: Option<String>,
    #[cli_docs(description = "The new owner of the account")]
    pub owner: Option<String>,
    #[cli_docs(description = "The new type of the account. Asset=0, Liability=1")]
    pub account_type: Option<AccountType>,
    #[cli_docs(description = "The new date and time the account was opened")]
    pub opened_on: Option<DateTime<Utc>>,
    #[cli_docs(description = "The new date and time the account was closed")]
    pub closed_on: Option<DateTime<Utc>>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CliDocs)]
pub struct CliCloseAccount {
    #[cli_docs(description = "The ID of the account")]
    pub id: i32,
    #[cli_docs(default = "Utc::now()", description = "The date and time the account was closed")]
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
    pub opened_on: NaiveDateTime,
    pub closed_on: Option<NaiveDateTime>,
}

impl From<CliOpenAccount> for InsertableAccount {
    fn from(value: CliOpenAccount) -> Self {
        Self {
            name: value.name,
            description: value.description,
            owner: value.owner.unwrap_or("me".to_string()),
            account_type: value.account_type.unwrap_or(AccountType::Asset) as i32,
            opened_on: value.opened_on.unwrap_or(Utc::now()).naive_utc(),
            closed_on: value.closed_on.map(|v| v.naive_utc()),
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
    pub opened_on: Option<NaiveDateTime>,
    pub closed_on: Option<NaiveDateTime>,
}

impl From<CliUpdateAccount> for UpdatableAccount {
    fn from(value: CliUpdateAccount) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            owner: value.owner,
            account_type: value.account_type.map(|v| v as i32),
            opened_on: value.opened_on.map(|v| v.naive_utc()),
            closed_on: value.closed_on.map(|v| v.naive_utc()),
        }
    }
}

#[derive(Debug, AsChangeset, Identifiable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=account)]
pub struct ClosableAccount {
    pub id: i32,
    pub closed_on: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=account)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub account_type: i32,
    pub opened_on: NaiveDateTime,
    pub closed_on: Option<NaiveDateTime>,
}
