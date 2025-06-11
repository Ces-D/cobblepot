use diesel::FromSqlRow;
use serde::{Deserialize, Serialize};

#[repr(i32)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, FromSqlRow)]
pub enum AccountType {
    Asset,
    Liability,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, FromSqlRow)]
pub enum RecurringStatus {
    Ongoing,
    Completed,
    Closed,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ReportType {
    BalanceSheet,
    DeepDiveAccount,
}

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub type CobblepotResult<T> = Result<T, CobblepotError>;

pub enum CobblepotError {
    DieselError(diesel::result::Error),
    JSONSerializationError(serde_json::Error),
    CliCommandError(&'static str),
    RruleError(rrule::RRuleError),
}

impl From<diesel::result::Error> for CobblepotError {
    fn from(err: diesel::result::Error) -> Self {
        CobblepotError::DieselError(err)
    }
}

impl From<serde_json::Error> for CobblepotError {
    fn from(err: serde_json::Error) -> Self {
        CobblepotError::JSONSerializationError(err)
    }
}

impl From<rrule::RRuleError> for CobblepotError {
    fn from(err: rrule::RRuleError) -> Self {
        CobblepotError::RruleError(err)
    }
}
