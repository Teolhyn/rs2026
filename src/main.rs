use axum::Router;
use dotenvy::dotenv;
use std::env;

use rs2026::db::establish_connection_pool;
use rs2026::{reservation, room, user};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_connection_pool(&database_url).expect("Failed to create connection pool");

    let app = Router::new()
        .merge(user::router())
        .merge(room::router())
        .merge(reservation::router())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
