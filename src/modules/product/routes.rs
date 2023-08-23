use actix_web::{delete, get, post, put, HttpResponse, web, Responder, Scope};

use super::data::{CreateProductRequest, FindProductResponse, UpdateProductRequest};
use crate::{AppState, utils::error::ResponseError};

pub fn routes() -> Scope {
    web::scope("/products")
        .service(save)
        .service(find_all)
        .service(find_by_id)
        .service(update)
        .service(delete_by_id)
}

#[post("")]
async fn save(dto: web::Json<CreateProductRequest>, state: web::Data<AppState>) -> impl Responder {
    let dto = dto.into_inner();
    let res = sqlx::query("INSERT INTO products (name, description) VALUES ($1, $2)")
        .bind(&dto.name)
        .bind(&dto.description)
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(err) => HttpResponse::BadRequest().json(ResponseError::from(err)),
    }
}

#[get("")]
async fn find_all(state: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query_as::<_, FindProductResponse>("SELECT id, name, description FROM products")
        .fetch_all(&state.pool)
        .await;

    match res {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => HttpResponse::BadRequest().json(ResponseError::from(err)),
    }
}

#[get("/{id}")]
async fn find_by_id(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let res = sqlx::query_as::<_, FindProductResponse>("SELECT id, name, description FROM products WHERE id=$1")
        .bind(id)
        .fetch_one(&state.pool)
        .await;

    match res {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(err) => {
            if let sqlx::Error::RowNotFound = err {
                HttpResponse::NotFound().json(ResponseError::from(format!("Product with id {} does not exist", id)))
            } else {
                HttpResponse::BadRequest().json(ResponseError::from(err))
            }
        }
    }
}

#[put("/{id}")]
async fn update(
    path: web::Path<i32>,
    dto: web::Json<UpdateProductRequest>,
    state: web::Data<AppState>,
    ) -> impl Responder {
    let id = path.into_inner();
    let dto = dto.into_inner();

    let res = sqlx::query("UPDATE products SET name=$2, description=$3 WHERE id=$1")
        .bind(id)
        .bind(&dto.name)
        .bind(&dto.description)
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
    let res = sqlx::query("DELETE FROM products WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(err) => HttpResponse::BadRequest().json(ResponseError::from(err)),
    }
}
