use crate::{schema::account, shared::AccountType};
use actix_web::{HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct JSONOpenAccount {
    pub name: String,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub account_type: Option<AccountType>,
    pub opened_on: Option<DateTime<Utc>>,
    pub closed_on: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct JSONUpdateAccount {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub account_type: Option<AccountType>,
    pub opened_on: Option<DateTime<Utc>>,
    pub closed_on: Option<DateTime<Utc>>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ToSchema)]
pub struct JSONCloseAccount {
    pub id: i32,
    #[schema(example = json!({"closed_on":Utc::now()}))]
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

impl From<JSONOpenAccount> for InsertableAccount {
    fn from(value: JSONOpenAccount) -> Self {
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

impl From<JSONUpdateAccount> for UpdatableAccount {
    fn from(value: JSONUpdateAccount) -> Self {
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

impl From<JSONCloseAccount> for ClosableAccount {
    fn from(value: JSONCloseAccount) -> Self {
        Self {
            id: value.id,
            closed_on: value.closed_on.map(|v| v.naive_utc()),
        }
    }
}

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, ToSchema)]
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

impl Responder for Account {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}
