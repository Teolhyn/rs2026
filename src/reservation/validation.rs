use chrono::{DateTime, Utc};

use crate::common::error::{AppError, ValidationError};
use crate::common::time::TimeSlot;
use crate::db::DbPool;
use crate::room::{self, RoomId};
use crate::user::{self, UserId};

pub struct ValidatedCreateReservation {
    pub room_id: RoomId,
    pub user_id: UserId,
    pub time_slot: TimeSlot,
}

impl ValidatedCreateReservation {
    pub fn new(
        pool: &DbPool,
        room_id: RoomId,
        user_id: UserId,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Self, AppError> {
        // Validate time slot (checks start < end and not in past)
        let time_slot = TimeSlot::new_future(start, end)?;

        // Validate room exists
        if !room::repository::room_exists(pool, room_id)? {
            return Err(ValidationError::InvalidRoomId.into());
        }

        // Validate user exists
        if !user::repository::user_exists(pool, user_id)? {
            return Err(ValidationError::InvalidUserId.into());
        }

        Ok(Self {
            room_id,
            user_id,
            time_slot,
        })
    }
}
