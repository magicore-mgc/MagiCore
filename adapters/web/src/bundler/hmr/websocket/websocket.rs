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

/// Whether the handshake may proceed: a browser must present a loopback
/// `Origin`, while a client that sends none is not a browser and already
/// reached this loopback listener directly.
/// Handshake có được tiếp tục không: trình duyệt phải gửi `Origin` loopback,
/// còn client không gửi Origin không phải trình duyệt và đã kết nối trực tiếp
/// tới listener loopback này.
///
/// The attack this blocks is a page the developer visits: it always sends
/// `Origin`. Refusing an absent header as well would break non-browser tools
/// without adding protection.
/// (Tấn công bị chặn là trang developer mở: nó luôn gửi `Origin`. Từ chối cả
/// trường hợp thiếu header sẽ phá công cụ không phải trình duyệt mà không thêm
/// bảo vệ.)
fn origin_is_loopback(headers: &axum::http::HeaderMap) -> bool {
    let Some(origin) = headers
        .get(axum::http::header::ORIGIN)
        .and_then(|value| value.to_str().ok())
    else {
        return headers.get(axum::http::header::ORIGIN).is_none();
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
