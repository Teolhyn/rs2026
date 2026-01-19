use diesel::prelude::*;

use crate::common::error::AppError;
use crate::db::schema::rooms;
use crate::db::DbPool;

use super::types::{NewRoom, Room, RoomFilter, RoomId};

pub fn create_room(pool: &DbPool, name: &str, capacity: i32) -> Result<Room, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    let new_room = NewRoom { name, capacity };

    diesel::insert_into(rooms::table)
        .values(&new_room)
        .returning(Room::as_returning())
        .get_result(&mut conn)
        .map_err(AppError::from)
}

pub fn get_room_by_id(pool: &DbPool, room_id: RoomId) -> Result<Room, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    rooms::table
        .find(room_id.0)
        .select(Room::as_select())
        .first(&mut conn)
        .map_err(AppError::from)
}

pub fn list_rooms(pool: &DbPool, filter: &RoomFilter) -> Result<Vec<Room>, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    let mut query = rooms::table.into_boxed();

    if let Some(min) = filter.min_capacity {
        query = query.filter(rooms::capacity.ge(min));
    }

    if let Some(max) = filter.max_capacity {
        query = query.filter(rooms::capacity.le(max));
    }

    query
        .select(Room::as_select())
        .load(&mut conn)
        .map_err(AppError::from)
}

pub fn room_exists(pool: &DbPool, room_id: RoomId) -> Result<bool, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    use diesel::dsl::exists;
    use diesel::select;

    select(exists(rooms::table.find(room_id.0)))
        .get_result(&mut conn)
        .map_err(AppError::from)
}
