use axum::{
    error_handling::HandleErrorLayer,
    extract::ws::WebSocketUpgrade,
    extract::State,
    http::StatusCode,
    response::Response,
    routing::{get, post},
    BoxError, Router,
};
use axum_macros::debug_handler;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tracing::*;

use crate::common;
use crate::config::SharedConfig;
use crate::function::{hook_up, AppState};

pub async fn start(config: SharedConfig) {
    let app_state = AppState { config };

    let app = Router::new()
        .route(
            // 認証
            "/",
            post(ws_post).get(ws_get).with_state(app_state.clone()),
        )
        .route(
            // バージョン
            "/version",
            get(common::version),
        )
        .layer(
            ServiceBuilder::new() // タイムアウト設定
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

#[debug_handler]
pub async fn ws_post(ws: WebSocketUpgrade, app_state: State<AppState>) -> Response {
    ws.on_upgrade(|socket| hook_up(socket, app_state))
}

#[debug_handler]
pub async fn ws_get(ws: WebSocketUpgrade, app_state: State<AppState>) -> Response {
    ws.on_upgrade(|socket| hook_up(socket, app_state))
}
