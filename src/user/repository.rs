use diesel::prelude::*;

use crate::db::schema::users;
use crate::db::DbPool;
use crate::{common::error::AppError, user::types::Email};

use super::types::{NewUser, User, UserId};

pub fn create_user(pool: &DbPool, email: &Email, name: &str) -> Result<User, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    let email = email.as_str();

    let new_user = NewUser { email, name };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .map_err(AppError::from)
}

pub fn get_user_by_id(pool: &DbPool, user_id: UserId) -> Result<User, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    users::table
        .find(user_id.0)
        .select(User::as_select())
        .first(&mut conn)
        .map_err(AppError::from)
}

pub fn user_exists(pool: &DbPool, user_id: UserId) -> Result<bool, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Database(e.to_string()))?;

    use diesel::dsl::exists;
    use diesel::select;

    select(exists(users::table.find(user_id.0)))
        .get_result(&mut conn)
        .map_err(AppError::from)
}

#[cfg(test)]
mod tests {

    use super::*;
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    pub fn test_pool() -> DbPool {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).unwrap();

        let mut conn = pool.get().unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();

        pool
    }

    #[test]
    fn test_create_user() {
        let pool = test_pool();

        let email = Email::parse("TEST@TeSt.com").unwrap();
        let uname = "Test Testson";

        let user = create_user(&pool, &email, uname).unwrap();

        assert_eq!(user.id, 1);
        assert_eq!(user.email, "test@test.com");
        assert_eq!(user.name, "Test Testson");
    }
}
