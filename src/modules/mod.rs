use actix_web::web;

pub mod user;
pub mod product;
pub mod auth;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(user::routes::routes())
        .service(product::routes::routes())
        .service(auth::routes::routes());
}
