use serde::{Deserialize, Serialize};

pub struct Product {
    id: Option<i32>,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct FindProductResponse {
    id: Option<i32>,
    name: String,
    description: Option<String>,

}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: String,
    pub description: Option<String>,
}
