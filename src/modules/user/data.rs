use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgQueryResult};

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

pub struct UserQuery;

impl UserQuery {
    pub async fn find_by_username(username: &str, pool: &PgPool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * from users where username=$1")
            .bind(&username)
            .fetch_one(pool)
            .await
    }

    pub async fn save(user: &User, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO users (name, email, username, password) values ($1, $2, $3, $4)")
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password)
            .execute(pool)
            .await
    }
}
