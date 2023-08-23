mod database;
mod modules;
mod utils;

use actix_web::{web, App, HttpServer, middleware::Logger};

struct AppState {
    pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenvy::dotenv().expect("Failed to load .env file");

    let pool = initialize_database().await;
    let state = web::Data::new(AppState { pool });

    HttpServer::new(move ||
                    App::new()
                    .wrap(Logger::default())
                    .app_data(state.clone())
                    .configure(modules::routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn initialize_database() -> sqlx::PgPool {
    let connection_string = std::env::var("DATABASE_URL").expect("Connection string not provided");
    let pool = database::connect(&connection_string)
        .await
        .expect("Failed to connect to database");
   database::migrate(&pool)
       .await
       .expect("Failed to migrate database");
    pool
}
