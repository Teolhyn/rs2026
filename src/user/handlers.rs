use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::common::error::AppError;
use crate::db::DbPool;
use crate::user::types::Email;

use super::repository;
use super::types::{CreateUserRequest, UserId, UserResponse};

pub fn router() -> Router<DbPool> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/{user_id}", get(get_user))
}

async fn create_user(
    State(pool): State<DbPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let email = Email::parse(&req.email)?;
    let user = repository::create_user(&pool, &email, &req.name)?;
    Ok(Json(user.into()))
}

async fn get_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponse>, AppError> {
    let user = repository::get_user_by_id(&pool, UserId(user_id))?;
    Ok(Json(user.into()))
}
