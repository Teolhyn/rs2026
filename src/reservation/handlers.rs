use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::common::error::AppError;
use crate::db::DbPool;
use crate::room::RoomId;
use crate::user::UserId;

use super::repository;
use super::types::{CreateReservationRequest, ReservationId, ReservationResponse};
use super::validation::ValidatedCreateReservation;

pub fn router() -> Router<DbPool> {
    Router::new()
        .route(
            "/rooms/{room_id}/reservations",
            post(create_reservation).get(list_reservations),
        )
        .route(
            "/rooms/{room_id}/reservations/{reservation_id}",
            delete(cancel_reservation),
        )
}

async fn create_reservation(
    State(pool): State<DbPool>,
    Path(room_id): Path<i32>,
    Json(req): Json<CreateReservationRequest>,
) -> Result<Json<ReservationResponse>, AppError> {
    let validated = ValidatedCreateReservation::new(
        &pool,
        RoomId(room_id),
        UserId(req.user_id),
        req.start_time,
        req.end_time,
    )?;

    let reservation = repository::create_reservation(&pool, validated)?;
    Ok(Json(reservation.into()))
}

async fn list_reservations(
    State(pool): State<DbPool>,
    Path(room_id): Path<i32>,
) -> Result<Json<Vec<ReservationResponse>>, AppError> {
    let reservations = repository::list_reservations_for_room(&pool, RoomId(room_id))?;
    Ok(Json(
        reservations
            .into_iter()
            .map(ReservationResponse::from)
            .collect(),
    ))
}

async fn cancel_reservation(
    State(pool): State<DbPool>,
    Path((room_id, reservation_id)): Path<(i32, i32)>,
) -> Result<Json<ReservationResponse>, AppError> {
    // First verify the reservation belongs to the room
    let reservation = repository::get_reservation_by_id(&pool, ReservationId(reservation_id))?;

    if reservation.room_id != room_id {
        return Err(AppError::NotFound(
            "Reservation not found in this room".to_string(),
        ));
    }

    let cancelled = repository::cancel_reservation(&pool, ReservationId(reservation_id))?;
    Ok(Json(cancelled.into()))
}
