use super::recurrance::Recurrance;
use crate::{schema::recurring_transactions, shared::AccountType};
use actix_web::{HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use chrono::NaiveDateTime;
use diesel::prelude::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONOpenRecurringTransaction {
    pub name: String,
    pub description: Option<String>,
    pub amount: f32,
    pub account_type: Option<AccountType>,
    pub recurrance: Recurrance,
    pub account_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONCloseRecurringTransaction {
    pub id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=recurring_transactions)]
pub struct InsertableRecurringTransaction {
    pub name: String,
    pub description: Option<String>,
    pub account_type: i32,
    pub amount: f32,
    pub rrule: String,
    pub start_date: NaiveDateTime,
    pub closed: bool,
    pub account_id: i32,
}

impl TryFrom<JSONOpenRecurringTransaction> for InsertableRecurringTransaction {
    type Error = rrule::RRuleError;

    fn try_from(value: JSONOpenRecurringTransaction) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            account_type: value.account_type.unwrap_or(AccountType::Asset) as i32,
            amount: value.amount,
            rrule: value.recurrance.rrule()?.to_string(),
            start_date: value.recurrance.dt_start.naive_utc(),
            closed: false,
            account_id: value.account_id,
        })
    }
}

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=recurring_transactions)]
pub struct RecurringTransaction {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub account_type: i32,
    pub amount: f32,
    pub rrule: String,
    pub start_date: NaiveDateTime,
    pub closed: bool,
    pub account_id: i32,
}

impl Responder for RecurringTransaction {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::iter::repeat_with;

    use crate::recurring_transaction::{
        model::JSONOpenRecurringTransaction, recurrance::test_utils::create_dummy_reccurance,
    };

    pub fn create_dummy_open_recurring_transaction(
        account_id: i32,
    ) -> JSONOpenRecurringTransaction {
        JSONOpenRecurringTransaction {
            name: repeat_with(fastrand::alphanumeric).take(10).collect(),
            description: None,
            amount: 100.0,
            account_type: None,
            recurrance: create_dummy_reccurance(),
            account_id,
        }
    }
}
