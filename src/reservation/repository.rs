use diesel::prelude::*;

use crate::common::error::{AppError, ConflictError};
use crate::db::schema::reservations;
use crate::db::DbPool;
use crate::room::RoomId;

use super::types::{NewReservation, Reservation, ReservationId, ReservationStatus};
use super::validation::ValidatedCreateReservation;

pub fn create_reservation(
    pool: &DbPool,
    validated: ValidatedCreateReservation,
) -> Result<Reservation, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    // Check for overlapping reservations
    let start_time = validated.time_slot.start().naive_utc();
    let end_time = validated.time_slot.end().naive_utc();

    let overlapping_exists: bool = reservations::table
        .filter(reservations::room_id.eq(validated.room_id.0))
        .filter(reservations::status.eq(ReservationStatus::Active.as_str()))
        .filter(reservations::start_time.lt(end_time))
        .filter(reservations::end_time.gt(start_time))
        .select(diesel::dsl::count_star())
        .first::<i64>(&mut conn)
        .map(|count| count > 0)
        .map_err(AppError::from)?;

    if overlapping_exists {
        return Err(ConflictError::OverlappingReservation.into());
    }

    let new_reservation = NewReservation {
        room_id: validated.room_id.0,
        user_id: validated.user_id.0,
        start_time,
        end_time,
        status: ReservationStatus::Active.as_str().to_string(),
    };

    diesel::insert_into(reservations::table)
        .values(&new_reservation)
        .returning(Reservation::as_returning())
        .get_result(&mut conn)
        .map_err(AppError::from)
}

pub fn get_reservation_by_id(
    pool: &DbPool,
    reservation_id: ReservationId,
) -> Result<Reservation, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    reservations::table
        .find(reservation_id.0)
        .select(Reservation::as_select())
        .first(&mut conn)
        .map_err(AppError::from)
}

pub fn list_reservations_for_room(
    pool: &DbPool,
    room_id: RoomId,
) -> Result<Vec<Reservation>, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    reservations::table
        .filter(reservations::room_id.eq(room_id.0))
        .select(Reservation::as_select())
        .order(reservations::start_time.asc())
        .load(&mut conn)
        .map_err(AppError::from)
}

pub fn cancel_reservation(
    pool: &DbPool,
    reservation_id: ReservationId,
) -> Result<Reservation, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    // First check the reservation exists
    let reservation = reservations::table
        .find(reservation_id.0)
        .select(Reservation::as_select())
        .first(&mut conn)
        .map_err(AppError::from)?;

    // Update status to cancelled
    diesel::update(reservations::table.find(reservation_id.0))
        .set(reservations::status.eq(ReservationStatus::Cancelled.as_str()))
        .execute(&mut conn)
        .map_err(AppError::from)?;

    // Return the updated reservation
    Ok(Reservation {
        status: ReservationStatus::Cancelled.as_str().to_string(),
        ..reservation
    })
}
