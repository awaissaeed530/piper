use actix_web::{post, web, Responder, Scope, HttpResponse};

use crate::{AppState, modules::user::data::{UserQuery, User}};
use super::data::{RegisterRequest, LoginRequest, LoginResponse, Claims, hash_password, verify_password, encode_token};

pub fn routes() -> Scope {
    web::scope("/auth")
        .service(register)
        .service(login)
}

#[post("register")]
async fn register(request: web::Json<RegisterRequest>, state: web::Data<AppState>) -> impl Responder {
    let dto = request.into_inner();
    let password = hash_password(&dto.password).unwrap();

    let user = User {
        id: None,
        name: dto.name,
        email: dto.email,
        username: dto.username,
        password,
    };
    let res = UserQuery::save(&user, &state.pool).await;

    match res {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::BadRequest()
    }
}

#[post("login")]
async fn login(request: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let dto = request.into_inner();
    let user = UserQuery::find_by_username(&dto.username, &state.pool).await;

    if user.is_err() {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }
    let user = user.unwrap();

    if verify_password(&dto.password, &user.password) {
        let claims = Claims {
            sub: user.id.unwrap().to_string(),
            name: user.name,
            email: user.email,
            username: user.username,
            exp: 10000 
        };
        let token = encode_token(&claims);
        let response = LoginResponse { token };
        return HttpResponse::Ok().json(response);
    }
    else {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }
}
