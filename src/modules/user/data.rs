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

pub struct UserQuery;

impl UserQuery {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * from users")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * from users where id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * from users where email=$1")
            .bind(&email)
            .fetch_one(pool)
            .await
    }

    pub async fn find_by_username(username: &str, pool: &PgPool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * from users where username=$1")
            .bind(&username)
            .fetch_one(pool)
            .await
    }


    pub async fn exists_by_id(id: i32, pool: &PgPool) -> Result<bool, sqlx::Error> {
        let res: (i32,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE id=$1)")
            .bind(id)
            .fetch_one(pool)
            .await?;

        if res.0 == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
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

    pub async fn update(
        id: i32,
        user: &User,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(
            "UPDATE users
            SET name=$2, email=$3
            WHERE id=$1",
        )
        .bind(id)
        .bind(&user.name)
        .bind(&user.email)
        .execute(pool)
        .await
    }

    pub async fn delete_by_id(
        id: i32,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await
    }
}
