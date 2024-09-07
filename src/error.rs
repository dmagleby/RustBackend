use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Not Found")]
    NotFound,
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error"),
            AppError::NotFound => HttpResponse::NotFound().json("Not Found"),
            AppError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

impl Default for AppError {
    fn default() -> Self {
        AppError::InternalServerError
    }
}