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
///
/// PAR-09: a WebSocket handshake is not protected by the same-origin policy,
/// so any page the developer visits could otherwise open the HMR socket and
/// observe project file paths and events. The upgrade is refused unless the
/// `Origin` header names a loopback host, which is the only kind of dev
/// server MagiCore starts.
/// (PAR-09: handshake WebSocket không được same-origin policy bảo vệ, nên bất
/// kỳ trang nào developer mở cũng có thể mở socket HMR và đọc đường dẫn file,
/// event của project. Upgrade bị từ chối trừ khi `Origin` là host loopback —
/// loại máy chủ dev duy nhất MagiCore khởi động.)
pub async fn hmr_ws_handler(
    headers: axum::http::HeaderMap,
    ws: WebSocketUpgrade,
    State(manager): State<Arc<HmrManager>>,
) -> impl IntoResponse {
    if !origin_is_loopback(&headers) {
        return axum::response::Response::builder()
            .status(axum::http::StatusCode::FORBIDDEN)
            .body(axum::body::Body::from(
                "HMR websocket refused: Origin must be a loopback dev server",
            ))
            .unwrap_or_else(|_| axum::response::Response::new(axum::body::Body::empty()));
    }
    ws.on_upgrade(|socket| handle_socket(socket, manager))
}

/// Whether the `Origin` header names a loopback host.
/// Header `Origin` có trỏ host loopback không.
///
/// A missing origin is refused too: only a browser dev page on this machine is
/// expected to connect.
/// (Origin thiếu cũng bị từ chối: chỉ trang dev trên máy này mới được nối.)
fn origin_is_loopback(headers: &axum::http::HeaderMap) -> bool {
    let Some(origin) = headers
        .get(axum::http::header::ORIGIN)
        .and_then(|value| value.to_str().ok())
    else {
        return false;
    };
    let host = origin
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(origin);
    let host = host.split('/').next().unwrap_or(host);
    let host = if let Some(rest) = host.strip_prefix('[') {
        rest.split(']').next().map(|inner| format!("[{inner}]"))
    } else {
        None
    }
    .unwrap_or_else(|| {
        host.rsplit_once(':')
            .filter(|(_, port)| port.chars().all(|c| c.is_ascii_digit()))
            .map(|(host, _)| host.to_string())
            .unwrap_or_else(|| host.to_string())
    });
    matches!(
        host.to_ascii_lowercase().as_str(),
        "localhost" | "127.0.0.1" | "[::1]"
    )
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
