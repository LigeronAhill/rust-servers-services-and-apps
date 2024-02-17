use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DbError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}
impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DbError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            EzyTutorError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            EzyTutorError::InvalidInput(msg) => {
                println!("Invalid parameters received: {msg:?}");
                msg.into()
            }
        }
    }
}

impl Display for EzyTutorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for EzyTutorError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            EzyTutorError::DbError(_msg) | EzyTutorError::ActixError(_msg) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(_msg) => actix_web::http::StatusCode::NOT_FOUND,
            EzyTutorError::InvalidInput(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl From<sqlx::Error> for EzyTutorError {
    fn from(error: sqlx::Error) -> Self {
        EzyTutorError::DbError(error.to_string())
    }
}
impl From<actix_web::Error> for EzyTutorError {
    fn from(error: actix_web::Error) -> Self {
        EzyTutorError::ActixError(error.to_string())
    }
}
