#[derive(serde::Serialize)]
pub struct ResponseError {
    pub message: String,
}

impl From<sqlx::Error> for ResponseError {
    fn from(err: sqlx::Error) -> Self {
        ResponseError {
            message: err.to_string()
        }
    }
}

impl From<String> for ResponseError {
    fn from(message: String) -> Self {
        ResponseError {
            message: message.to_owned()
        }
    }
}
