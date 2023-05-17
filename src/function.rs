use axum::{extract::ws::WebSocket, extract::State};
use tracing::*;

use crate::config::SharedConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: SharedConfig,
}

#[instrument]
pub async fn hook_up(stream: WebSocket, app_state: State<AppState>) {}
