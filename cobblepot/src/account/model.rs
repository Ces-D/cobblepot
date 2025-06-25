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

#[derive(Debug, AsChangeset, Identifiable)]
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
    pub closed_on: NaiveDateTime,
}

impl From<JSONCloseAccount> for ClosableAccount {
    fn from(value: JSONCloseAccount) -> Self {
        Self {
            id: value.id,
            closed_on: value.closed_on.unwrap_or(Utc::now()).naive_utc(),
        }
    }
}

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize, ToSchema)]
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

#[cfg(test)]
pub mod test_utils {
    use chrono::{Months, Utc};

    use crate::account::model::{Account, JSONOpenAccount, JSONUpdateAccount};
    use crate::shared::AccountType;
    use std::iter::repeat_with;

    pub fn create_dummy_open_account() -> JSONOpenAccount {
        JSONOpenAccount {
            name: repeat_with(fastrand::alphanumeric).take(10).collect(),
            description: None,
            owner: None,
            account_type: None,
            opened_on: None,
            closed_on: None,
        }
    }

    /// Changes all but the `id`, `name`, and `closed_on` fields of the original account
    pub fn create_dummy_update_account(account: &Account) -> JSONUpdateAccount {
        JSONUpdateAccount {
            id: account.id,
            name: None,
            description: Some(repeat_with(fastrand::alphanumeric).take(10).collect()),
            owner: Some(repeat_with(fastrand::alphanumeric).take(10).collect()),
            account_type: match AccountType::from(account.account_type) {
                AccountType::Asset => Some(AccountType::Liability),
                AccountType::Liability => Some(AccountType::Asset),
            },
            opened_on: Some(Utc::now().checked_sub_months(Months::new(6)).unwrap()),
            closed_on: None,
        }
    }
}
