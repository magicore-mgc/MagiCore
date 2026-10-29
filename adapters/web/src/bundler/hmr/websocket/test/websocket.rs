//! HMR event JSON payload tests.
//! Test JSON payload event HMR.

use super::{event_payload, origin_is_loopback};
use crate::bundler::hmr::HmrEvent;

#[test]
fn encodes_reload_event_with_the_existing_message_shape() {
    let payload = event_payload(HmrEvent::Reload);

    assert_eq!(payload, serde_json::json!({ "type": "reload" }).to_string());
}

#[test]
fn encodes_update_path_as_json_data() {
    let payload = event_payload(HmrEvent::Update("src/main.ts".to_string()));
    let value: serde_json::Value = serde_json::from_str(&payload).unwrap();

    assert_eq!(value["type"], "update");
    assert_eq!(value["path"], "src/main.ts");
}

/// PAR-09: only a loopback dev page may open the HMR socket.
/// (PAR-09: chỉ trang dev loopback được mở socket HMR.)
#[test]
fn only_loopback_origins_may_open_the_hmr_socket() {
    use axum::http::{HeaderMap, HeaderValue, header::ORIGIN};

    let with_origin = |value: &str| {
        let mut headers = HeaderMap::new();
        headers.insert(ORIGIN, HeaderValue::from_str(value).unwrap());
        headers
    };

    for allowed in [
        "http://localhost:3000",
        "http://127.0.0.1:5173",
        "https://localhost",
        "http://[::1]:8080",
    ] {
        assert!(
            origin_is_loopback(&with_origin(allowed)),
            "{allowed} is a loopback dev page"
        );
    }
    for denied in [
        "https://evil.example",
        "http://192.168.1.5:3000",
        "http://localhost.evil.example",
        "null",
    ] {
        assert!(
            !origin_is_loopback(&with_origin(denied)),
            "{denied} must be refused"
        );
    }
    // A client with no Origin header is not a browser page and is already
    // connected to the loopback listener, so it may proceed.
    // (Client không có header Origin không phải trang trình duyệt và đã nối tới
    // listener loopback, nên được tiếp tục.)
    assert!(origin_is_loopback(&HeaderMap::new()));
}
