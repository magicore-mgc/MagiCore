//! HMR WebSocket connection and event protocol.
//! Kết nối WebSocket HMR và event protocol.

use std::sync::Arc;

use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};

use super::super::{event::HmrEvent, manager::HmrManager};

/// Upgrade the registered HMR endpoint and serve one client connection.
/// Upgrade endpoint HMR đã đăng ký và phục vụ một client connection.
pub async fn hmr_ws_handler(
    ws: WebSocketUpgrade,
    State(manager): State<Arc<HmrManager>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, manager))
}

async fn handle_socket(mut socket: WebSocket, manager: Arc<HmrManager>) {
    let mut receiver = manager.subscribe();
    let greeting = serde_json::json!({ "type": "connected" }).to_string();
    if socket.send(Message::Text(greeting.into())).await.is_err() {
        return;
    }

    loop {
        tokio::select! {
            event = receiver.recv() => {
                match event {
                    Ok(event) => {
                        let message = Message::Text(event_payload(event).into());
                        if socket.send(message).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            message = socket.recv() => {
                if let Some(Ok(Message::Close(_))) = message {
                    break;
                }
                if message.is_none() {
                    break;
                }
            }
        }
    }
}

fn event_payload(event: HmrEvent) -> String {
    match event {
        HmrEvent::Reload => serde_json::json!({ "type": "reload" }).to_string(),
        HmrEvent::Update(path) => serde_json::json!({ "type": "update", "path": path }).to_string(),
    }
}

#[cfg(test)]
#[path = "test/websocket.rs"]
mod tests;
