use actix_web::{
    HttpResponse, ResponseError,
    error::BlockingError,
    http::{StatusCode, header::ContentType},
};
use derive_more::Display;

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
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for CobblepotError {
    fn from(err: reqwest::Error) -> Self {
        CobblepotError::ReqwestError(err)
    }
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
            CobblepotError::ReqwestError(err) => match err.status() {
                Some(status) => StatusCode::from_u16(status.as_u16())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                None => StatusCode::INTERNAL_SERVER_ERROR,
            },
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
            CobblepotError::ReqwestError(e) => build_response(e),
        }
    }
}
