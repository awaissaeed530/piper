use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect(connection_string: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect(connection_string)
        .await
}

pub async fn setup(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await.expect("Failed to start transaction");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(60) NOT NULL,
        email VARCHAR(60) NOT NULL,
        username VARCHAR(30) NOT NULL UNIQUE,
        password VARCHAR(255) NOT NULL);"
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS products (
        id SERIAL PRIMARY KEY,
        name VARCHAR(60) NOT NULL,
        description TEXT);"
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await
}
