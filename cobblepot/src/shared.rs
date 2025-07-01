use actix_web::{
    HttpResponse, ResponseError,
    error::BlockingError,
    http::{StatusCode, header::ContentType},
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    Asset = 0,
    Liability = 1,
}

impl From<i32> for AccountType {
    fn from(value: i32) -> Self {
        match value {
            0 => AccountType::Asset,
            1 => AccountType::Liability,
            _ => panic!("Invalid AccountType value"),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Frequency {
    Yearly = 0,
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
    Hourly = 4,
    Minutely = 5,
    Secondly = 6,
}

impl Into<rrule::Frequency> for Frequency {
    fn into(self) -> rrule::Frequency {
        match self {
            Frequency::Yearly => rrule::Frequency::Yearly,
            Frequency::Monthly => rrule::Frequency::Monthly,
            Frequency::Weekly => rrule::Frequency::Weekly,
            Frequency::Daily => rrule::Frequency::Daily,
            Frequency::Hourly => rrule::Frequency::Hourly,
            Frequency::Minutely => rrule::Frequency::Minutely,
            Frequency::Secondly => rrule::Frequency::Secondly,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Weekday {
    Mon = 0,
    Tue = 1,
    Wed = 2,
    Thu = 3,
    Fri = 4,
    Sat = 5,
    Sun = 6,
}

impl Into<rrule::Weekday> for Weekday {
    fn into(self) -> rrule::Weekday {
        match self {
            Weekday::Mon => rrule::Weekday::Mon,
            Weekday::Tue => rrule::Weekday::Tue,
            Weekday::Wed => rrule::Weekday::Wed,
            Weekday::Thu => rrule::Weekday::Thu,
            Weekday::Fri => rrule::Weekday::Fri,
            Weekday::Sat => rrule::Weekday::Sat,
            Weekday::Sun => rrule::Weekday::Sun,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RecurringStatus {
    Ongoing = 0,
    Completed = 1,
    Closed = 2,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ReportType {
    BalanceSheet = 0,
    DeepDiveAccount = 1,
}

pub type CobblepotResult<T> = Result<T, CobblepotError>;

#[derive(Debug, Display)]
pub enum CobblepotError {
    DieselError(diesel::result::Error),
    EnvError(std::env::VarError),
    /// App logic error
    LogicError(String),
    /// App user error
    UserError(String),
    IoError(std::io::Error),
}

impl From<diesel::result::Error> for CobblepotError {
    fn from(err: diesel::result::Error) -> Self {
        CobblepotError::DieselError(err)
    }
}

impl From<serde_json::Error> for CobblepotError {
    fn from(err: serde_json::Error) -> Self {
        CobblepotError::UserError(err.to_string())
    }
}

impl From<rrule::RRuleError> for CobblepotError {
    fn from(err: rrule::RRuleError) -> Self {
        CobblepotError::LogicError(err.to_string())
    }
}

impl From<std::env::VarError> for CobblepotError {
    fn from(err: std::env::VarError) -> Self {
        CobblepotError::EnvError(err)
    }
}

impl From<std::io::Error> for CobblepotError {
    fn from(err: std::io::Error) -> Self {
        CobblepotError::IoError(err)
    }
}

impl From<BlockingError> for CobblepotError {
    fn from(err: BlockingError) -> Self {
        CobblepotError::LogicError(err.to_string())
    }
}

impl ResponseError for CobblepotError {
    fn status_code(&self) -> StatusCode {
        match self {
            CobblepotError::DieselError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CobblepotError::EnvError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CobblepotError::LogicError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CobblepotError::UserError(_) => StatusCode::BAD_REQUEST,
            CobblepotError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let build_response = |error: &dyn std::fmt::Display| {
            HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(error.to_string())
        };

        match self {
            CobblepotError::DieselError(e) => build_response(e),
            CobblepotError::EnvError(e) => build_response(e),
            CobblepotError::LogicError(e) => build_response(e),
            CobblepotError::UserError(e) => build_response(e),
            CobblepotError::IoError(e) => build_response(e),
        }
    }
}
