//! HMR event JSON payload tests.
//! Test JSON payload event HMR.

use super::event_payload;
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
