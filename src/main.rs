mod modules;
mod database;

use actix_web::{web, App, HttpServer, middleware::Logger};
use modules::{user::routes::user_routes, product::routes::product_routes, auth::routes::auth_routes};
use sqlx::PgPool;
use env_logger::Env;

struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenvy::dotenv().expect("Failed to load .env file");

    let pool = initialize_database().await;
    let state = web::Data::new(AppState { pool });

    HttpServer::new(move ||
                    App::new()
                    .wrap(Logger::default())
                    .app_data(state.clone())
                    .service(user_routes())
                    .service(product_routes())
                    .service(auth_routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn initialize_database() -> PgPool {
    let connection_string = std::env::var("DATABASE_URL").expect("Connection string not provided");
    let pool = database::connect(&connection_string)
        .await
        .expect("Failed to connect to database");
   database::migrate(&pool)
       .await
       .expect("Failed to migrate database");
    pool
}
