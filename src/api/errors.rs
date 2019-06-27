use actix_web::{http, HttpResponse, ResponseError};
use std::fmt::Display;

#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    Unauthorized,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            ServiceError::BadRequest(ref inner) => ::std::fmt::Display::fmt(inner, f),
            ServiceError::Unauthorized => f.write_str("Attetion Criminal you are not Unauthorized"),
            ServiceError::InternalServerError => {
                f.write_str("Internal Server Error, Please try later")
            }
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            ServiceError::Unauthorized => HttpResponse::new(http::StatusCode::UNAUTHORIZED),

            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}
