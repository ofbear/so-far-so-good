use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    BoxError, Router,
};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tracing::*;

use crate::common;
use crate::config::SharedConfig;
use crate::healthcheck::{status, SharedHealthcheck};
use crate::http;
use crate::ws;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: SharedConfig,
    pub healthcheck: SharedHealthcheck,
}

pub async fn start(config: SharedConfig, healthcheck: SharedHealthcheck) {
    let app_state = AppState {
        config,
        healthcheck,
    };

    let app = Router::new()
        .route(
            // 認証
            "/http",
            post(http::post)
                .get(http::get)
                .with_state(app_state.clone()),
        )
        .route(
            "/ws",
            post(ws::post).get(ws::get).with_state(app_state.clone()),
        )
        .route("/version", get(common::version))
        .route("/healthcheck", get(status).with_state(app_state.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::SERVICE_UNAVAILABLE
                }))
                .layer(TimeoutLayer::new(app_state.config.server.timeout)),
        );

    info!("Inquiring.");
    axum::Server::bind(&app_state.config.server.addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|e| panic!("The dream is gone.:{}", e));
}
