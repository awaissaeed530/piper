mod modules;

use core::panic;

use actix_web::{web, App, HttpServer};
use modules::user::routes::user_routes;
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteQueryResult},
    SqlitePool,
};

struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await;
    let pool = match pool {
        Ok(_pool) => _pool,
        Err(err) => {
            panic!("{:?}", err);
        }
    };

    match setup(&pool).await {
        Ok(_) => {}
        Err(err) => {
            panic!("{:?}", err);
        }
    };

    let state = web::Data::new(AppState { pool });

    HttpServer::new(move || App::new().app_data(state.clone()).service(user_routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn setup(pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE users (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        name TEXT NOT NULL,
        email TEXT NOT NULL)",
    )
    .execute(pool)
    .await
}
