use actix_web::{post, web, Responder, Scope, HttpResponse};

use crate::{AppState, modules::user::data::User, utils::error::ResponseError};
use super::data::{RegisterRequest, LoginRequest, LoginResponse, Claims, hash_password, verify_password, encode_token};

pub fn routes() -> Scope {
    web::scope("/auth")
        .service(register)
        .service(login)
}

#[post("register")]
async fn register(dto: web::Json<RegisterRequest>, state: web::Data<AppState>) -> impl Responder {
    let dto = dto.into_inner();
    let password = hash_password(&dto.password).unwrap();

    let res = sqlx::query("INSERT INTO users (name, email, username, password) values ($1, $2, $3, $4)")
        .bind(&dto.name)
        .bind(&dto.email)
        .bind(&dto.username)
        .bind(&password)
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(err) => HttpResponse::BadRequest().json(ResponseError::from(err))
    }
}

#[post("login")]
async fn login(dto: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let dto = dto.into_inner();
    let error = ResponseError::from("Invalid username or password".to_owned());

    let res = sqlx::query_as::<_, User>("SELECT id, name, email, username, password FROM users WHERE username=$1")
        .bind(&dto.username)
        .fetch_one(&state.pool)
        .await;

    let user = match res {
        Ok(_user) => _user,
        Err(_) => {
            return HttpResponse::Unauthorized().json(error);
        }
    };

    if verify_password(&dto.password, &user.password) {
        let claims = Claims {
            sub: user.id.unwrap_or_default().to_string(),
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
        return HttpResponse::Unauthorized().json(error);
    }
}
