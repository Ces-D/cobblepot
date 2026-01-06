use chrono::{DateTime, NaiveDateTime};
use diesel::{
    AsExpression, FromSqlRow, Selectable,
    backend::Backend,
    deserialize::{self, FromSql},
    prelude::{Identifiable, Queryable},
    serialize::{self, Output, ToSql},
    sql_types::Integer,
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

pub mod schema;

/// A wrapper type that converts between NaiveDateTime and UNIX timestamp (Integer in SQLite)
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Integer)]
pub struct UnixTimestamp(pub NaiveDateTime);

impl UnixTimestamp {
    pub fn new(dt: NaiveDateTime) -> Self {
        UnixTimestamp(dt)
    }

    pub fn inner(&self) -> NaiveDateTime {
        self.0
    }
}

impl<DB> FromSql<Integer, DB> for UnixTimestamp
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let timestamp = i32::from_sql(bytes)?;
        let dt = DateTime::from_timestamp(timestamp as i64, 0)
            .ok_or_else(|| "Invalid timestamp")?
            .naive_utc();
        Ok(UnixTimestamp(dt))
    }
}

impl ToSql<Integer, Sqlite> for UnixTimestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let timestamp = self.0.and_utc().timestamp() as i32;
        out.set_value(timestamp);
        Ok(serialize::IsNull::No)
    }
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
    pub starts_on: UnixTimestamp,
    pub ends_on: Option<UnixTimestamp>,
    pub recurrence_rule: Option<String>,
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
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=schema::budget_item_account)]
pub struct BudgetItemAccount {
    pub account_id: i32,
    pub budget_item_id: i32,
    pub allocation_percentage: Option<i32>,
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
