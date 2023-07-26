use core::panic;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteQueryResult},
    SqlitePool,
};

struct AppState {
    pool: SqlitePool,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
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

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(save)
            .service(find_all)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/user")]
async fn save(user: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query("INSERT INTO users (name, email) values ($1, $2)")
        .bind(user.name.to_owned())
        .bind(user.email.to_owned())
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[get("/user")]
async fn find_all(state: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query_as::<_, User>("SELECT * from users")
        .fetch_all(&state.pool)
        .await;

    match res {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::BadRequest().body(""),
    }
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
