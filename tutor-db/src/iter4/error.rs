use actix_web::{HttpResponse, error, http::StatusCode};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;

#[derive(Debug, Deserialize, Serialize)]
pub enum EzytutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Deserialize, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::fmt::Display for EzytutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EzytutorError::DBError(msg) => write!(f, "Database error: {}", msg),
            EzytutorError::ActixError(msg) => write!(f, "Internal server error: {}", msg),
            EzytutorError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl EzytutorError {
    pub fn error_response(&self) -> String {
        match self {
            EzytutorError::DBError(msg) => {
                error!("Database Error occurred {:?}", msg);
                "Database error".into()
            }
            EzytutorError::ActixError(msg) => {
                error!("Server Error occurred {:?}", msg);
                "Internal server error".into()
            }
            EzytutorError::NotFound(msg) => {
                error!("Not Found Error occurred {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for EzytutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzytutorError::DBError(_) | EzytutorError::ActixError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzytutorError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.to_string(),
        })
    }
}

impl From<SqlxError> for EzytutorError {
    fn from(err: SqlxError) -> Self {
        EzytutorError::DBError(err.to_string())
    }
}
