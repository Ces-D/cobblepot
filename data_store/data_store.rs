use diesel::{
    Selectable,
    prelude::{Identifiable, Queryable},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

pub mod schema;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub enum RecurringStatus {
    #[default]
    Ongoing = 0,
    Completed = 1,
    Closed = 2,
}

/// Represents an account as it is stored in the database.
#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::account)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub account_type: i32,
    pub opened_on: chrono::NaiveDateTime,
    pub closed_on: Option<chrono::NaiveDateTime>,
}

/// Represents a balance record as it is stored in the database.
#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::balance)]
pub struct Balance {
    pub id: i32,
    pub memo: String,
    pub amount: f32,
    pub entered_on: chrono::NaiveDateTime,
    pub account_id: i32,
}

/// Represents a financial market instrument as it is stored in the database.
#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::market_instrument)]
pub struct MarketInstrument {
    pub id: i32,
    pub name: String,
    pub ticker: String,
    pub market: Option<String>,
    pub instrument_type: i32,
    pub quantity: f32,
    pub opened_on: chrono::NaiveDateTime,
    pub updated_on: chrono::NaiveDateTime,
    pub account_id: i32,
}

/// Represents a recurring transaction as it is stored in the database.
#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::recurring_transactions)]
pub struct RecurringTransaction {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub account_type: i32,
    pub amount: f32,
    pub rrule: String,
    pub start_date: chrono::NaiveDateTime,
    pub closed: bool,
    pub account_id: i32,
}
