use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ErrorKind {
    #[error("invalid request")]
    InvalidRequest,
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // здесь можно обозначить свою структуру
        // и проконтролить что мы возвращаем на клиент как есть а что нет

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}
