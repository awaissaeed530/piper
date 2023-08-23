use actix_web::{delete, get, put, HttpResponse, web, Responder, Scope};

use super::data::{FindUserResponse, UpdateUserRequest};
use crate::{AppState, utils::error::ResponseError};

pub fn routes() -> Scope {
    web::scope("/users")
        .service(find_all)
        .service(find_by_id)
        .service(update)
        .service(delete_by_id)
}

#[get("")]
async fn find_all(state: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query_as::<_, FindUserResponse>("SELECT id, name, email, username FROM users")
        .fetch_all(&state.pool)
        .await;

    match res {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::BadRequest().json(ResponseError::from(err)),
    }
}

#[get("/{id}")]
async fn find_by_id(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let res = sqlx::query_as::<_, FindUserResponse>("SELECT id, name, email, username FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(&state.pool)
        .await;

    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            if let sqlx::Error::RowNotFound = err {
                HttpResponse::NotFound().json(ResponseError::from(format!("User with id {} does not exist", id)))
            } else {
                HttpResponse::BadRequest().json(ResponseError::from(err))
            }
        }
    }
}

#[put("/{id}")]
async fn update(
    path: web::Path<i32>,
    dto: web::Json<UpdateUserRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let res = sqlx::query("UPDATE users SET name=$2 WHERE id=$1")
        .bind(id)
        .bind(&dto.name)
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(err) => HttpResponse::BadRequest().json(ResponseError::from(err)),
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
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(err) => HttpResponse::BadRequest().json(ResponseError::from(err)),
    }
}
