use crate::{
    schema::market_instrument,
    shared::{InstrumentType, responder::impl_json_responder},
};
use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{
    Identifiable, Insertable, Queryable, Selectable, prelude::AsChangeset, sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

/// Represents the payload for a web form to open a new financial market instrument.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONOpenFinancialMarketInstrument {
    name: String,
    ticker: String,
    market: Option<String>,
    instrument_type: InstrumentType,
    quantity: f32,
    opened_on: Option<DateTime<Utc>>,
    account_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONUpdateFinancialMarketInstrument {
    pub id: i32,
    name: Option<String>,
    ticker: Option<String>,
    market: Option<String>,
    quantity: Option<f32>,
    account_id: Option<i32>,
}

/// Represents a new financial market instrument to be inserted into the database.
#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=market_instrument)]
pub struct InsertableMarketInstrument {
    name: String,
    ticker: String,
    market: Option<String>,
    instrument_type: i32,
    quantity: f32,
    opened_on: NaiveDateTime,
    updated_on: NaiveDateTime,
    account_id: i32,
}

/// Represents a new financial market instrument to be inserted into the database.
#[derive(Debug, AsChangeset, Identifiable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=market_instrument)]
pub struct UpdatableMarketInstrument {
    id: i32,
    name: Option<String>,
    ticker: Option<String>,
    market: Option<String>,
    quantity: Option<f32>,
    updated_on: NaiveDateTime,
    account_id: Option<i32>,
}

impl From<JSONUpdateFinancialMarketInstrument> for UpdatableMarketInstrument {
    fn from(value: JSONUpdateFinancialMarketInstrument) -> Self {
        UpdatableMarketInstrument {
            id: value.id,
            name: value.name,
            ticker: value.ticker,
            market: value.market,
            quantity: value.quantity,
            updated_on: chrono::Utc::now().naive_utc(),
            account_id: value.account_id,
        }
    }
}

impl From<JSONOpenFinancialMarketInstrument> for InsertableMarketInstrument {
    fn from(value: JSONOpenFinancialMarketInstrument) -> Self {
        InsertableMarketInstrument {
            name: value.name,
            ticker: value.ticker,
            market: value.market,
            instrument_type: value.instrument_type as i32,
            quantity: value.quantity,
            opened_on: value
                .opened_on
                .map(|v| v.naive_utc())
                .unwrap_or_else(|| Utc::now().naive_utc()),
            updated_on: Utc::now().naive_utc(),
            account_id: value.account_id,
        }
    }
}

/// Represents a financial market instrument as it is stored in the database.
#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=market_instrument)]
pub struct MarketInstrument {
    pub id: i32,
    pub name: String,
    pub ticker: String,
    pub market: Option<String>,
    pub instrument_type: i32,
    pub quantity: f32,
    pub opened_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub account_id: i32,
}

/// A list of market instruments, used for responding to API requests.
#[derive(Debug, Serialize)]
pub struct MarketInstrumentList(pub Vec<MarketInstrument>);

impl_json_responder!(MarketInstrumentList);

/// Represents a calculated financial market instrument, enriched with external data.
/// This struct contains fields similar to `MarketInstrument` but is designed for display,
/// featuring a calculated `total_value` from an external API like Tiingo and user-friendly data types.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalculatedMarketInstrument {
    pub id: i32,
    pub name: String,
    pub ticker: String,
    pub market: Option<String>,
    pub instrument_type: InstrumentType,
    pub quantity: f32,
    pub opened_on: DateTime<Utc>,
    pub account_id: i32,
    pub total_value: f64,
}

impl From<MarketInstrument> for CalculatedMarketInstrument {
    fn from(value: MarketInstrument) -> Self {
        Self {
            id: value.id,
            name: value.name,
            ticker: value.ticker,
            market: value.market,
            instrument_type: InstrumentType::Stock,
            quantity: value.quantity,
            opened_on: Utc::now(),
            account_id: value.account_id,
            total_value: 0.0,
        }
    }
}

impl_json_responder!(CalculatedMarketInstrument);
