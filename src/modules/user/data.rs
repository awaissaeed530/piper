use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

pub struct UserQuery;

impl UserQuery {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * from users")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * from users where id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn find_by_email(email: String, pool: &SqlitePool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * from users where email=$1")
            .bind(email)
            .fetch_one(pool)
            .await
    }

    pub async fn exists_by_id(id: i32, pool: &SqlitePool) -> Result<bool, sqlx::Error> {
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

    pub async fn save(user: User, pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO users (name, email) values ($1, $2)")
            .bind(user.name.to_owned())
            .bind(user.email.to_owned())
            .execute(pool)
            .await
    }

    pub async fn update(
        id: i32,
        user: User,
        pool: &SqlitePool,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "UPDATE users
            SET name=$2, email=$3
            WHERE id=$1",
        )
        .bind(id)
        .bind(user.name.to_owned())
        .bind(user.email.to_owned())
        .execute(pool)
        .await
    }

    pub async fn delete_by_id(
        id: i32,
        pool: &SqlitePool,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await
    }
}
