use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Product {
    id: Option<i32>,
    name: String,
    description: Option<String>,
}

pub struct ProductQuery;

impl ProductQuery {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * from products")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Product, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * from products where id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn exists_by_id(id: i32, pool: &PgPool) -> Result<bool, sqlx::Error> {
        let res: (i32,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM products WHERE id=$1)")
            .bind(id)
            .fetch_one(pool)
            .await?;

        if res.0 == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn save(product: &Product, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO products (name, description) values ($1, $2)")
            .bind(&product.name)
            .bind(&product.description)
            .execute(pool)
            .await
    }

    pub async fn update(
        id: i32,
        product: &Product,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(
            "UPDATE products
            SET name=$2, description=$3
            WHERE id=$1",
        )
        .bind(id)
        .bind(&product.name)
        .bind(&product.description)
        .execute(pool)
        .await
    }

    pub async fn delete_by_id(
        id: i32,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM products WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await
    }
}
