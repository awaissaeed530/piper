use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect(connection_string: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect(connection_string)
        .await
}

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!().run(pool).await
}
