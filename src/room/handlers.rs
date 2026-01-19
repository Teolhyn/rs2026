use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::common::error::AppError;
use crate::db::DbPool;

use super::repository;
use super::types::{CreateRoomRequest, RoomFilter, RoomResponse};

pub fn router() -> Router<DbPool> {
    Router::new()
        .route("/rooms", post(create_room))
        .route("/rooms", get(list_rooms))
}

async fn create_room(
    State(pool): State<DbPool>,
    Json(req): Json<CreateRoomRequest>,
) -> Result<Json<RoomResponse>, AppError> {
    let room = repository::create_room(&pool, &req.name, req.capacity)?;
    Ok(Json(room.into()))
}

async fn list_rooms(
    State(pool): State<DbPool>,
    Query(filter): Query<RoomFilter>,
) -> Result<Json<Vec<RoomResponse>>, AppError> {
    let rooms = repository::list_rooms(&pool, &filter)?;
    Ok(Json(rooms.into_iter().map(RoomResponse::from).collect()))
}
