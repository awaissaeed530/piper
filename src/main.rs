use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteQueryResult},
    SqlitePool,
};

#[derive(sqlx::FromRow, Debug)]
struct User {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;
    setup(&pool).await?;

    let user: User = User {
        name: "Awais Saeed".to_owned(),
        email: "awais@mail.com".to_owned(),
    };
    save(user, &pool).await?;
    let users = find_all(&pool).await?;
    println!("{:?}", users);
    Ok(())
}

async fn save(user: User, pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) values ($1, $2)")
        .bind(user.name)
        .bind(user.email)
        .execute(pool)
        .await
}

async fn find_all(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * from users")
        .fetch_all(pool)
        .await
}

async fn setup(pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE users (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        name TEXT NOT NULL,
        email TEXT NOT NULL)",
    )
    .execute(pool)
    .await
}
