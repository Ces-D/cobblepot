use diesel::{
    Selectable,
    prelude::{Identifiable, Queryable},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

mod recurrence_rule;
pub mod schema;
mod unix_timestamp;
pub use recurrence_rule::RecurrenceRule;
pub use unix_timestamp::UnixTimestamp;

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
    pub opened_on: UnixTimestamp,
    pub closed_on: Option<UnixTimestamp>,
}

/// Represents a balance record as it is stored in the database.
#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::balance)]
pub struct Balance {
    pub id: i32,
    pub memo: String,
    pub amount: f32,
    pub entered_on: UnixTimestamp,
    pub account_id: i32,
}

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::budget)]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub anticipated_amount: f32,
    pub budget_recurrence_id: Option<i32>,
}

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::budget_item)]
pub struct BudgetItem {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub amount: f32,
    pub budget_id: i32,
    pub budget_recurrence_id: Option<i32>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::budget_item_account)]
pub struct BudgetItemAccount {
    pub account_id: i32,
    pub budget_item_id: i32,
    pub allocation_percentage: Option<i32>,
}

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::budget_recurrence)]
pub struct BudgetRecurrence {
    pub id: i32,
    pub dt_start: UnixTimestamp,
    pub recurrence_rule: RecurrenceRule,
    pub budget_id: Option<i32>,
    pub budget_item_id: Option<i32>,
}

// TODO:
// Represents a financial market instrument as it is stored in the database.
// #[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize, Clone)]
// #[diesel(check_for_backend(Sqlite))]
// #[diesel(table_name=schema::market_instrument)]
// pub struct MarketInstrument {
//     pub id: i32,
//     pub name: String,
//     pub ticker: String,
//     pub market: Option<String>,
//     pub instrument_type: i32,
//     pub quantity: f32,
//     pub opened_on: chrono::NaiveDateTime,
//     pub updated_on: chrono::NaiveDateTime,
//     pub account_id: i32,
// }
