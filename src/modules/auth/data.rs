use argon2::{Argon2, password_hash::{SaltString, rand_core::OsRng}, PasswordHasher, PasswordHash, PasswordVerifier};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub username: String,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash_password).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn encode_token(claims: &Claims) -> String {
    let secret = jsonwebtoken::EncodingKey::from_secret("secret".as_ref());
    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &secret);
    token.unwrap()
}

pub fn decode_token(token: &str) -> Claims {
    let secret = jsonwebtoken::DecodingKey::from_secret("secret".as_ref());
    let claims = jsonwebtoken::decode::<Claims>(token, &secret, &jsonwebtoken::Validation::default());
    claims.unwrap().claims
}
