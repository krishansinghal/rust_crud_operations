use actix_web::{HttpResponse, ResponseError}; // For creating HTTP responses and handling errors in Actix Web
use thiserror::Error; // For defining custom errors using the `thiserror` crate

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),  // Error when interacting with the database

    #[error("Invalid ID format: {0}")]
    InvalidIdFormat(String),  // Error for invalid ObjectId format

    #[error("Document not found")]
    DocumentNotFound,  // When a document is not found in the database
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),  // New error for invalid inputs
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().body(self.to_string()),  // Handle database error
            AppError::InvalidIdFormat(_) => HttpResponse::BadRequest().body(self.to_string()),  // Handle invalid ID format error
            AppError::DocumentNotFound => HttpResponse::NotFound().body(self.to_string()),  // Handle document not found error
            AppError::InvalidInput(_) => HttpResponse::BadRequest().body(self.to_string()),  // Handle invalid input error
        }
    }
}
