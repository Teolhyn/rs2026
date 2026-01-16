use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::rooms;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(pub i32);

impl From<i32> for RoomId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<RoomId> for i32 {
    fn from(id: RoomId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = rooms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub capacity: i32,
    pub created_at: NaiveDateTime,
}

impl Room {
    pub fn room_id(&self) -> RoomId {
        RoomId(self.id)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = rooms)]
pub struct NewRoom<'a> {
    pub name: &'a str,
    pub capacity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub capacity: i32,
}

#[derive(Debug, Serialize)]
pub struct RoomResponse {
    pub id: i32,
    pub name: String,
    pub capacity: i32,
    pub created_at: String,
}

impl From<Room> for RoomResponse {
    fn from(room: Room) -> Self {
        Self {
            id: room.id,
            name: room.name,
            capacity: room.capacity,
            created_at: room.created_at.and_utc().to_rfc3339(),
        }
    }
}
