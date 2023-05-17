use axum::{
    extract::ws::Message, extract::ws::WebSocket, extract::ws::WebSocketUpgrade, extract::Query,
    extract::State, response::Response, Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use tracing::*;

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::server::AppState;

const TIMESPAN: Duration = Duration::from_millis(10);

#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    _session_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InputData {
    words: Vec<String>,
}

#[debug_handler(state = AppState)]
pub async fn post(
    ws: WebSocketUpgrade,
    app_state: State<AppState>,
    Json(request): Json<Request>,
) -> Response {
    ws.on_upgrade(|socket| hook_up(socket, app_state, request))
}

#[debug_handler(state = AppState)]
pub async fn get(
    ws: WebSocketUpgrade,
    app_state: State<AppState>,
    Query(request): Query<Request>,
) -> Response {
    ws.on_upgrade(|socket| hook_up(socket, app_state, request))
}

#[instrument]
pub async fn hook_up(stream: WebSocket, _app_state: State<AppState>, _request: Request) {
    let (sender_to_outside, receiver_from_outside) = stream.split();
    let (sender_to_inside, receiver_from_inside) = mpsc::unbounded_channel::<String>();
    let (sender_to_entrance, receiver_from_entrance) = mpsc::unbounded_channel::<String>();
    let is_loop = Arc::new(Mutex::new(true));

    tokio::join!(
        entrance(receiver_from_outside, sender_to_inside, &is_loop),
        inside(receiver_from_inside, sender_to_entrance, &is_loop),
        exit(receiver_from_entrance, sender_to_outside, &is_loop),
    );
}

async fn entrance(
    mut receiver_from_outside: SplitStream<WebSocket>,
    sender_to_inside: mpsc::UnboundedSender<String>,
    is_loop: &Arc<Mutex<bool>>,
) {
    let mut interval = tokio::time::interval(TIMESPAN);
    loop {
        interval.tick().await;
        if !*is_loop.lock().await {
            break;
        }

        if let Ok(v) = timeout(TIMESPAN, receiver_from_outside.next()).await {
            if let Some(Ok(msg)) = v {
                if let Message::Text(txt) = msg {
                    if let Ok(input_data) = serde_json::from_str::<InputData>(&txt) {
                        info!("{:?}", input_data);
                        for word in input_data.words {
                            if let Err(e) = sender_to_inside.send(word) {
                                error!("It's a test from God.: {:?}", e);
                                *is_loop.lock().await = false;
                                break;
                            }
                        }
                    }
                } else if let Message::Binary(_data) = msg {
                    info!("do something");
                    continue;
                }
            } else {
                *is_loop.lock().await = false;
                info!("websocket strangely died.");
                break;
            }
        }
    }

    info!("I reached the goal.");
}

async fn exit(
    mut receiver_from_entrance: mpsc::UnboundedReceiver<String>,
    mut sender_to_outside: SplitSink<WebSocket, Message>,
    is_loop: &Arc<Mutex<bool>>,
) {
    let mut interval = tokio::time::interval(TIMESPAN);
    loop {
        interval.tick().await;
        if !*is_loop.lock().await {
            break;
        }

        if let Ok(v) = timeout(TIMESPAN, receiver_from_entrance.recv()).await {
            if let Some(txt) = v {
                if let Err(e) = sender_to_outside.send(Message::from(txt)).await {
                    error!("A strange thing happened then.: {:?}", e);
                    *is_loop.lock().await = false;
                    break;
                }
            } else {
                *is_loop.lock().await = false;
                info!("websocket strangely died.");
                break;
            }
        }
    }

    info!("I reached the goal.");
}

async fn inside(
    mut receiver_from_inside: mpsc::UnboundedReceiver<String>,
    sender_to_entrance: mpsc::UnboundedSender<String>,
    is_loop: &Arc<Mutex<bool>>,
) {
    let mut interval = tokio::time::interval(TIMESPAN);
    loop {
        interval.tick().await;
        if !*is_loop.lock().await {
            break;
        }

        if let Ok(v) = timeout(TIMESPAN, receiver_from_inside.recv()).await {
            if let Some(txt) = v {
                if let Err(e) = sender_to_entrance.send(txt) {
                    error!("A strange thing happened then.: {:?}", e);
                    *is_loop.lock().await = false;
                    break;
                }
            } else {
                *is_loop.lock().await = false;
                info!("websocket strangely died.");
                break;
            }
        }
    }

    info!("I reached the goal.");
}
