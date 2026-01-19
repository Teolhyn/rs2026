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

    conn.exclusive_transaction(|conn| {
        let overlapping_exists: bool = reservations::table
            .filter(reservations::room_id.eq(validated.room_id.0))
            .filter(reservations::status.eq(ReservationStatus::Active.as_str()))
            .filter(reservations::start_time.lt(end_time))
            .filter(reservations::end_time.gt(start_time))
            .select(diesel::dsl::count_star())
            .first::<i64>(conn)
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
            .get_result(conn)
            .map_err(AppError::from)
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::SqliteConnection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    use crate::common::error::ConflictError;
    use crate::common::time::TimeSlot;
    use crate::room::repository::create_room;
    use crate::user::repository::create_user;
    use crate::user::types::Email;
    use crate::user::UserId;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    fn test_pool() -> DbPool {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).unwrap();

        let mut conn = pool.get().unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();

        pool
    }

    fn create_test_validated_reservation(
        room_id: RoomId,
        user_id: UserId,
        start_hours_from_now: i64,
        duration_hours: i64,
    ) -> ValidatedCreateReservation {
        let start = Utc::now() + Duration::hours(start_hours_from_now);
        let end = start + Duration::hours(duration_hours);
        let time_slot = TimeSlot::new(start, end).unwrap();

        ValidatedCreateReservation {
            room_id,
            user_id,
            time_slot,
        }
    }

    #[test]
    fn test_create_reservation() {
        let pool = test_pool();

        let room = create_room(&pool, "Huone", 10).unwrap();
        let email = Email::parse("test@test.com").unwrap();
        let user = create_user(&pool, &email, "Testi Käyttäjä").unwrap();

        let validated = create_test_validated_reservation(RoomId(room.id), UserId(user.id), 1, 2);

        let reservation = create_reservation(&pool, validated).unwrap();

        assert_eq!(reservation.id, 1);
        assert_eq!(reservation.room_id, room.id);
        assert_eq!(reservation.user_id, user.id);
        assert_eq!(reservation.status, "active");
    }

    #[test]
    fn test_create_reservation_overlapping() {
        let pool = test_pool();

        let room = create_room(&pool, "Huone", 10).unwrap();
        let email = Email::parse("test@test.com").unwrap();
        let user = create_user(&pool, &email, "Testi Käyttäjä").unwrap();

        // Create first reservation: hours 1-3
        let validated1 = create_test_validated_reservation(RoomId(room.id), UserId(user.id), 1, 2);
        create_reservation(&pool, validated1).unwrap();

        // Try to create overlapping reservation: hours 2-4
        let validated2 = create_test_validated_reservation(RoomId(room.id), UserId(user.id), 2, 2);
        let result = create_reservation(&pool, validated2);

        assert!(matches!(
            result,
            Err(AppError::Conflict(ConflictError::OverlappingReservation))
        ));
    }

    #[test]
    fn test_get_reservation_by_id() {
        let pool = test_pool();

        let room = create_room(&pool, "Huone", 10).unwrap();
        let email = Email::parse("test@test.com").unwrap();
        let user = create_user(&pool, &email, "Testi Käyttäjä").unwrap();

        let validated = create_test_validated_reservation(RoomId(room.id), UserId(user.id), 1, 2);
        let created = create_reservation(&pool, validated).unwrap();

        let fetched = get_reservation_by_id(&pool, ReservationId(created.id)).unwrap();

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.room_id, room.id);
    }

    #[test]
    fn test_list_reservations_for_room() {
        let pool = test_pool();

        let room1 = create_room(&pool, "Huone 1", 10).unwrap();
        let room2 = create_room(&pool, "Huone 2", 10).unwrap();
        let email = Email::parse("test@test.com").unwrap();
        let user = create_user(&pool, &email, "Testi Käyttäjä").unwrap();

        // Create reservations for room1
        let v1 = create_test_validated_reservation(RoomId(room1.id), UserId(user.id), 1, 1);
        let v2 = create_test_validated_reservation(RoomId(room1.id), UserId(user.id), 3, 1);
        create_reservation(&pool, v1).unwrap();
        create_reservation(&pool, v2).unwrap();

        // Create reservation for room2
        let v3 = create_test_validated_reservation(RoomId(room2.id), UserId(user.id), 1, 1);
        create_reservation(&pool, v3).unwrap();

        let room1_reservations = list_reservations_for_room(&pool, RoomId(room1.id)).unwrap();
        let room2_reservations = list_reservations_for_room(&pool, RoomId(room2.id)).unwrap();

        assert_eq!(room1_reservations.len(), 2);
        assert_eq!(room2_reservations.len(), 1);
    }

    #[test]
    fn test_cancel_reservation() {
        let pool = test_pool();

        let room = create_room(&pool, "Huone", 10).unwrap();
        let email = Email::parse("test@test.com").unwrap();
        let user = create_user(&pool, &email, "Testi Käyttäjä").unwrap();

        let validated = create_test_validated_reservation(RoomId(room.id), UserId(user.id), 1, 2);
        let reservation = create_reservation(&pool, validated).unwrap();

        assert_eq!(reservation.status, "active");

        let cancelled = cancel_reservation(&pool, ReservationId(reservation.id)).unwrap();

        assert_eq!(cancelled.status, "cancelled");
    }

    #[test]
    fn test_cancelled_reservation_does_not_block_new() {
        let pool = test_pool();

        let room = create_room(&pool, "Huone", 10).unwrap();
        let email = Email::parse("test@test.com").unwrap();
        let user = create_user(&pool, &email, "Testi Käyttäjä").unwrap();

        // Create and cancel a reservation
        let validated1 = create_test_validated_reservation(RoomId(room.id), UserId(user.id), 1, 2);
        let reservation = create_reservation(&pool, validated1).unwrap();
        cancel_reservation(&pool, ReservationId(reservation.id)).unwrap();

        // Create new reservation for same time slot - should succeed
        let validated2 = create_test_validated_reservation(RoomId(room.id), UserId(user.id), 1, 2);
        let new_reservation = create_reservation(&pool, validated2);

        assert!(new_reservation.is_ok());
    }
}
