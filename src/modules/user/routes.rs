use actix_web::{delete, get, post, web, Responder, Scope};
use actix_web::{put, HttpResponse};

use super::data::{User, UserQuery};
use crate::AppState;

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
    let res = UserQuery::save(user.into_inner(), &state.pool).await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[get("")]
async fn find_all(state: web::Data<AppState>) -> impl Responder {
    let res = UserQuery::find_all(&state.pool).await;

    match res {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::BadRequest().body(""),
    }
}

#[get("/{id}")]
async fn find_by_id(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let res = UserQuery::find_by_id(id, &state.pool).await;

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
    let res = UserQuery::update(id, user.into_inner(), &state.pool).await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[delete("/{id}")]
async fn delete_by_id(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let res = UserQuery::delete_by_id(id, &state.pool).await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest(),
    }
}
