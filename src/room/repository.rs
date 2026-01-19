use diesel::prelude::*;

use crate::common::error::{AppError, ValidationError};
use crate::db::schema::rooms;
use crate::db::DbPool;

use super::types::{NewRoom, Room, RoomFilter, RoomId};

pub fn create_room(pool: &DbPool, name: &str, capacity: i32) -> Result<Room, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    if capacity > 1000 {
        return Err(AppError::Validation(ValidationError::CapacityTooLarge));
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::SqliteConnection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    fn test_pool() -> DbPool {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).unwrap();

        let mut conn = pool.get().unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();

        pool
    }

    #[test]
    fn test_create_room() {
        let pool = test_pool();

        let room = create_room(&pool, "Neuvotteluhuone A", 10).unwrap();

        assert_eq!(room.id, 1);
        assert_eq!(room.name, "Neuvotteluhuone A");
        assert_eq!(room.capacity, 10);
    }

    #[test]
    fn test_create_room_capacity_too_large() {
        let pool = test_pool();

        let result = create_room(&pool, "Suuri sali", 1001);

        assert!(matches!(
            result,
            Err(AppError::Validation(ValidationError::CapacityTooLarge))
        ));
    }

    #[test]
    fn test_get_room_by_id() {
        let pool = test_pool();

        let created = create_room(&pool, "Huone B", 5).unwrap();
        let fetched = get_room_by_id(&pool, RoomId(created.id)).unwrap();

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "Huone B");
    }

    #[test]
    fn test_list_rooms_no_filter() {
        let pool = test_pool();

        create_room(&pool, "Huone 1", 5).unwrap();
        create_room(&pool, "Huone 2", 10).unwrap();
        create_room(&pool, "Huone 3", 15).unwrap();

        let filter = RoomFilter {
            min_capacity: None,
            max_capacity: None,
        };
        let rooms = list_rooms(&pool, &filter).unwrap();

        assert_eq!(rooms.len(), 3);
    }

    #[test]
    fn test_list_rooms_with_min_capacity() {
        let pool = test_pool();

        create_room(&pool, "Pieni", 5).unwrap();
        create_room(&pool, "Keskikokoinen", 10).unwrap();
        create_room(&pool, "Suuri", 20).unwrap();

        let filter = RoomFilter {
            min_capacity: Some(10),
            max_capacity: None,
        };
        let rooms = list_rooms(&pool, &filter).unwrap();

        assert_eq!(rooms.len(), 2);
        assert!(rooms.iter().all(|r| r.capacity >= 10));
    }

    #[test]
    fn test_list_rooms_with_max_capacity() {
        let pool = test_pool();

        create_room(&pool, "Pieni", 5).unwrap();
        create_room(&pool, "Keskikokoinen", 10).unwrap();
        create_room(&pool, "Suuri", 20).unwrap();

        let filter = RoomFilter {
            min_capacity: None,
            max_capacity: Some(10),
        };
        let rooms = list_rooms(&pool, &filter).unwrap();

        assert_eq!(rooms.len(), 2);
        assert!(rooms.iter().all(|r| r.capacity <= 10));
    }

    #[test]
    fn test_room_exists() {
        let pool = test_pool();

        let room = create_room(&pool, "Huone", 10).unwrap();

        assert!(room_exists(&pool, RoomId(room.id)).unwrap());
        assert!(!room_exists(&pool, RoomId(999)).unwrap());
    }
}
