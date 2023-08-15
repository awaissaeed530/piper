mod modules;

use actix_web::{web, App, HttpServer, middleware::Logger};
use modules::{user::routes::user_routes, product::routes::product_routes};
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteQueryResult},
    SqlitePool,
};
use env_logger::Env;

struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to initialize db pool");
    setup(&pool).await.expect("Failed to initialize database");

    let state = web::Data::new(AppState { pool });

    HttpServer::new(move || 
                    App::new()
                    .wrap(Logger::default())
                    .app_data(state.clone())
                    .service(user_routes())
                    .service(product_routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn setup(pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE users (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        name TEXT NOT NULL,
        email TEXT NOT NULL);

        CREATE TABLE products (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        name TEXT NOT NULL,
        description TEXT);",
    )
    .execute(pool)
    .await
}
