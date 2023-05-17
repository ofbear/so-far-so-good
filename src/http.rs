use anyhow::Result;
use axum::{extract::Query, extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tracing::*;

use crate::error::AppError;
use crate::server::AppState;

#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    _session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub result: String,
}

#[debug_handler(state = AppState)]
pub async fn post(app_state: State<AppState>, Json(request): Json<Request>) -> impl IntoResponse {
    hook_up(request.clone(), app_state).await
}

#[debug_handler(state = AppState)]
pub async fn get(app_state: State<AppState>, request: Query<Request>) -> impl IntoResponse {
    hook_up(request.0.clone(), app_state).await
}

#[instrument(skip(_app_state))]
pub async fn hook_up(
    request: Request,
    _app_state: State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    Ok((
        StatusCode::OK,
        Json(Response {
            result: request._session_id,
        }),
    ))
}
