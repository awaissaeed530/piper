use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct FindUserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
}
