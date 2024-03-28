use actix_multipart::MultipartError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::{Map as JsonMap, Value as JsonValue};
use sqlx::Error as PgError;
use std::{convert::From, string::FromUtf8Error, io::Error as ioError};
use thiserror::Error as ThisError;
use validator::ValidationErrors;
use jsonwebtoken::errors::Error as JwtError;
#[derive(ThisError, Debug)]
pub enum AppError {
    // 400
    #[error("BadRequest: {0}")]
    BadRequest(JsonValue),

    // 401
    #[error("Unauthorized: {0}")]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {0}")]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {0}")]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            AppError::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            AppError::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            AppError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            AppError::UnprocessableEntity(ref message) => {
                HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message)
            }
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        }
    }
}

impl From<PgError> for AppError {
    fn from(err: PgError) -> Self {
        match err {
            PgError::Database(db_err) => {
                eprintln!("Database Error: {:?}", db_err);

                AppError::InternalServerError
            }

            _ => {
                println!("{}", err);
                AppError::InternalServerError},
        }
    }
}

impl From<JwtError> for AppError{
    fn from(value: JwtError) -> Self {
        AppError::UnprocessableEntity(value.to_string().into())
    }
}

impl From<FromUtf8Error> for AppError{
    fn from(_value: FromUtf8Error) -> Self {
        AppError::InternalServerError
    }
}

impl From<ioError> for AppError{
    fn from(_value: ioError) -> Self {
        AppError::UnprocessableEntity("cant write file".into())
    }
}

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        // Your conversion logic here
        // For example, you might have a variant of AppError specifically for multipart errors
        AppError::UnprocessableEntity(err.to_string().into()) // Assuming AppError has a variant for MultipartError
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        // Initialize an empty map to hold the simplified error messages
        let mut simplified_errors = serde_json::Map::new();

        // Iterate over the field errors
        for (field, field_errors) in errors.field_errors() {
            // For each field, we'll concatenate all its errors into a single string message
            let error_messages = field_errors
                .iter()
                .map(|error| {
                    // Each error might have a detailed message, but for simplicity, we use the error kind's name
                    // You might need to adjust this part to match your error handling
                    format!("{}", error.code)
                })
                .collect::<Vec<String>>()
                .join(", ");

            // Insert the field and its concatenated error message into our map
            simplified_errors.insert(field.to_string(), serde_json::Value::String(error_messages));
        }

        // Construct the final error with the simplified error messages
        AppError::UnprocessableEntity(serde_json::json!({ "validation_errors": simplified_errors }))
    }
}
