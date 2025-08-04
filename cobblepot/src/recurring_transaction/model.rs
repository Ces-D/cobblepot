use super::recurrance::Recurrance;
use crate::{
    schema::recurring_transactions,
    shared::{AccountType, responder::impl_json_responder},
};
use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use chrono::NaiveDateTime;
use diesel::prelude::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

/// Represents the JSON payload for listing recurring transactions with optional filters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONListRecurringTransactions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub account_id: Option<i32>,
}

/// Represents the JSON payload for opening a new recurring transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONOpenRecurringTransaction {
    pub name: String,
    pub description: Option<String>,
    pub amount: f32,
    pub account_type: Option<AccountType>,
    pub recurrance: Recurrance,
    pub account_id: i32,
}

/// Represents the JSON payload for closing a recurring transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONCloseRecurringTransaction {
    pub id: i32,
}

/// Represents a new recurring transaction to be inserted into the database.
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

/// Represents a recurring transaction as it is stored in the database.
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

impl_json_responder!(RecurringTransaction);

/// A list of recurring transactions, used for responding to API requests.
#[derive(Debug, Serialize)]
pub struct RecurringTransactionList(pub Vec<RecurringTransaction>);

impl_json_responder!(RecurringTransactionList);

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
