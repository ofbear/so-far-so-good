use anyhow::{Context, Result};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_macros::debug_handler;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Mutex;
use tracing::*;

use crate::server::AppState;

#[derive(Debug, Clone)]
pub struct Healthcheck {
    pub is_sigterm: bool,
}

pub type SharedHealthcheck = Arc<Mutex<Healthcheck>>;

impl Healthcheck {
    pub fn new() -> SharedHealthcheck {
        Arc::new(Mutex::new(Self { is_sigterm: false }))
    }
}

#[debug_handler(state = AppState)]
#[instrument(skip(app_state))]
pub async fn status(app_state: State<AppState>) -> impl IntoResponse {
    if app_state.healthcheck.lock().await.is_sigterm {
        StatusCode::IM_USED
    } else {
        StatusCode::OK
    }
}

pub async fn monitoring(healthcheck: SharedHealthcheck) -> Result<()> {
    let mut stream = signal(SignalKind::terminate())?;
    let _ = stream.recv().await.context("failed to get a signal stream");
    healthcheck.lock().await.is_sigterm = true;
    info!("get sigterm");
    Ok(())
}
