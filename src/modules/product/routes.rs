use actix_web::{delete, get, post, web, Responder, Scope};
use actix_web::{put, HttpResponse};

use super::data::{Product, ProductQuery};
use crate::AppState;

pub fn product_routes() -> Scope {
    web::scope("/products")
        .service(save)
        .service(find_all)
        .service(find_by_id)
        .service(update)
        .service(delete_by_id)
}

#[post("")]
async fn save(product: web::Json<Product>, state: web::Data<AppState>) -> impl Responder {
    let res = ProductQuery::save(&product.into_inner(), &state.pool).await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[get("")]
async fn find_all(state: web::Data<AppState>) -> impl Responder {
    let res = ProductQuery::find_all(&state.pool).await;

    match res {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::BadRequest().body(""),
    }
}

#[get("/{id}")]
async fn find_by_id(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let res = ProductQuery::find_by_id(id, &state.pool).await;

    match res {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(err) => {
            if let sqlx::Error::RowNotFound = err {
                return HttpResponse::NotFound().body(format!("Product with id {} does not exist", id));
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
    product: web::Json<Product>,
    state: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let res = ProductQuery::update(id, &product.into_inner(), &state.pool).await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[delete("/{id}")]
async fn delete_by_id(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let res = ProductQuery::delete_by_id(id, &state.pool).await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest(),
    }
}
