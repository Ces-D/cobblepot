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
    JsonError(serde_json::Error),
    RRuleError(rrule::RRuleError),
    BlockingError(BlockingError),
    DieselR2D2Error(diesel::r2d2::Error),
    R2D2Error(r2d2::Error),
}

impl From<diesel::r2d2::Error> for CobblepotError {
    fn from(err: diesel::r2d2::Error) -> Self {
        CobblepotError::DieselR2D2Error(err)
    }
}

impl From<r2d2::Error> for CobblepotError {
    fn from(err: r2d2::Error) -> Self {
        CobblepotError::R2D2Error(err)
    }
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
        CobblepotError::JsonError(err)
    }
}

impl From<rrule::RRuleError> for CobblepotError {
    fn from(err: rrule::RRuleError) -> Self {
        CobblepotError::RRuleError(err)
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
        CobblepotError::BlockingError(err)
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
            CobblepotError::JsonError(_) => StatusCode::BAD_REQUEST,
            CobblepotError::RRuleError(_) => StatusCode::BAD_REQUEST,
            CobblepotError::BlockingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CobblepotError::R2D2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CobblepotError::DieselR2D2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let error_message = self.to_string();

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(error_message)
    }
}
