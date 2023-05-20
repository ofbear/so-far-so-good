use axum::{http::StatusCode, response::IntoResponse};
use tracing::*;

#[derive(Debug)]
pub enum AppError {
    IncorrectRequest,
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        info!("{:?}", self);
        let (status_code, message) = match &self {
            AppError::IncorrectRequest => (StatusCode::BAD_REQUEST, "incorrect request"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
        };
        (status_code, message).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        info!("{:?}", e);
        AppError::InternalServerError
    }
}
