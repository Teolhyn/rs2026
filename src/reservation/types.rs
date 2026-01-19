use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::time::TimeSlot;
use crate::db::schema::reservations;
use crate::room::RoomId;
use crate::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReservationId(pub i32);

impl From<i32> for ReservationId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<ReservationId> for i32 {
    fn from(id: ReservationId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservationStatus {
    Active,
    Cancelled,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseReservationStatusError;

impl ReservationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Cancelled => "cancelled",
        }
    }
}

impl FromStr for ReservationStatus {
    type Err = ParseReservationStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(ParseReservationStatusError),
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = reservations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Reservation {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub status: String,
    pub created_at: NaiveDateTime,
}

impl Reservation {
    pub fn reservation_id(&self) -> ReservationId {
        ReservationId(self.id)
    }

    pub fn room_id(&self) -> RoomId {
        RoomId(self.room_id)
    }

    pub fn user_id(&self) -> UserId {
        UserId(self.user_id)
    }

    pub fn status(&self) -> ReservationStatus {
        ReservationStatus::from_str(&self.status).expect("Invalid status in database!")
    }

    pub fn time_slot(&self) -> TimeSlot {
        TimeSlot::new(self.start_time.and_utc(), self.end_time.and_utc())
            .expect("Database should contain valid time slots")
    }

    pub fn is_active(&self) -> bool {
        self.status() == ReservationStatus::Active
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = reservations)]
pub struct NewReservation {
    pub room_id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReservationRequest {
    pub user_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ReservationResponse {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub created_at: String,
}

impl From<Reservation> for ReservationResponse {
    fn from(reservation: Reservation) -> Self {
        Self {
            id: reservation.id,
            room_id: reservation.room_id,
            user_id: reservation.user_id,
            start_time: reservation.start_time.and_utc().to_rfc3339(),
            end_time: reservation.end_time.and_utc().to_rfc3339(),
            status: reservation.status,
            created_at: reservation.created_at.and_utc().to_rfc3339(),
        }
    }
}
