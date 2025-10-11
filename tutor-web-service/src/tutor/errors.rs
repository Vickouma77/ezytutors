use actix_web::{HttpResponse, Result, error, http::StatusCode};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum EzytutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}
impl std::error::Error for EzytutorError {}

impl EzytutorError {
    fn error_response(&self) -> String {
        match self {
            EzytutorError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            EzytutorError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzytutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for EzytutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzytutorError::DBError(_msg) | EzytutorError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzytutorError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for EzytutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            EzytutorError::DBError(msg) => write!(f, "Database error: {}", msg),
            EzytutorError::ActixError(msg) => write!(f, "Actix error: {}", msg),
            EzytutorError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl From<actix_web::error::Error> for EzytutorError {
    fn from(err: actix_web::error::Error) -> Self {
        EzytutorError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for EzytutorError {
    fn from(err: SQLxError) -> Self {
        EzytutorError::DBError(err.to_string())
    }
}
