use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Conflict: {0}")]
    Conflict(#[from] ConflictError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(String),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("End time must be after start time")]
    EndBeforeStart,

    #[error("Reservation cannot be made in the past")]
    ReservationInPast,

    #[error("Invalid room ID")]
    InvalidRoomId,

    #[error("Invalid user ID")]
    InvalidUserId,

    #[error("Invalid email")]
    InvalidEmail,

    #[error("Capacity too large")]
    CapacityTooLarge,
}

#[derive(Debug, Error)]
pub enum ConflictError {
    #[error("Overlapping reservation exists")]
    OverlappingReservation,

    #[error("Email already exists")]
    EmailAlreadyExists,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::Validation(ValidationError::EndBeforeStart) => {
                (StatusCode::BAD_REQUEST, "INVALID_TIME_RANGE")
            }
            AppError::Validation(ValidationError::ReservationInPast) => {
                (StatusCode::BAD_REQUEST, "RESERVATION_IN_PAST")
            }
            AppError::Validation(ValidationError::InvalidRoomId) => {
                (StatusCode::BAD_REQUEST, "INVALID_ROOM_ID")
            }
            AppError::Validation(ValidationError::InvalidUserId) => {
                (StatusCode::BAD_REQUEST, "INVALID_USER_ID")
            }
            AppError::Validation(ValidationError::InvalidEmail) => {
                (StatusCode::BAD_REQUEST, "INVALID_EMAIL")
            }
            AppError::Validation(ValidationError::CapacityTooLarge) => {
                (StatusCode::BAD_REQUEST, "CAPACITY_TOO_LARGE")
            }
            AppError::Conflict(ConflictError::OverlappingReservation) => {
                (StatusCode::CONFLICT, "OVERLAPPING_RESERVATION")
            }
            AppError::Conflict(ConflictError::EmailAlreadyExists) => {
                (StatusCode::CONFLICT, "EMAIL_ALREADY_EXISTS")
            }
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
            code: code.to_string(),
        });

        (status, body).into_response()
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound("Resource not found".to_string()),
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => AppError::Conflict(ConflictError::EmailAlreadyExists),
            _ => AppError::Database(err.to_string()),
        }
    }
}
