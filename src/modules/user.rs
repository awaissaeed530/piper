use actix_web::{get, post, web, Responder, Scope, delete};
use actix_web::{put, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

pub fn user_routes() -> Scope {
    web::scope("/users")
        .service(save)
        .service(find_all)
        .service(find_by_id)
        .service(update)
        .service(delete_by_id)
}

#[post("")]
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

#[get("")]
async fn find_all(state: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query_as::<_, User>("SELECT * from users")
        .fetch_all(&state.pool)
        .await;

    match res {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::BadRequest().body(""),
    }
}

#[get("/{id}")]
async fn find_by_id(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let res = sqlx::query_as::<_, User>("SELECT * from users where id=$1")
        .bind(id)
        .fetch_one(&state.pool)
        .await;

    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            if let sqlx::Error::RowNotFound = err {
                return HttpResponse::NotFound().body(format!("Row with id {} does not exist", id));
            } else {
                return HttpResponse::BadRequest()
                    .body(format!("Unhandled error {}", err.to_string()));
            }
        }
    }
}

#[put("/{id}")]
async fn update(
    path: web::Path<i32>,
    user: web::Json<User>,
    state: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let res = sqlx::query(
        "UPDATE users
        SET name=$2, email=$3
        WHERE id=$1",
    )
    .bind(id)
    .bind(user.name.to_owned())
    .bind(user.email.to_owned())
    .execute(&state.pool)
    .await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[delete("/{id}")]
async fn delete_by_id(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let res = sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest(),
    }
}
