use std::fmt;

use actix_web::{HttpResponse, error, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum EzytutorError {
    DBError(SqlxError),
    ActixError(String),
    NotFound(String),
}

impl Serialize for EzytutorError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EzytutorError {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialization not supported for errors
        Err(serde::de::Error::custom("Cannot deserialize EzytutorError"))
    }
}

#[derive(Deserialize, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl fmt::Display for EzytutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EzytutorError::DBError(err) => write!(f, "Database error: {}", err),
            EzytutorError::ActixError(msg) => write!(f, "Internal server error: {}", msg),
            EzytutorError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for EzytutorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EzytutorError::DBError(err) => Some(err),
            EzytutorError::ActixError(_) => None,
            EzytutorError::NotFound(_) => None,
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
        EzytutorError::DBError(err)
    }
}
