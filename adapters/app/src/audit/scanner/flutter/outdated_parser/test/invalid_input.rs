//! Flutter outdated parser fail-closed input cases.
//! Case parser Flutter từ chối input lỗi.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn flutter_outdated_garbage_fails_closed() {
    assert!(parse_flutter_outdated_json("}}} nonsense").is_err());
}

#[test]
fn flutter_outdated_missing_groups_fails_closed() {
    // A payload with NO recognized group is not a pub outdated report —
    // schema error, never "zero drift".
    // Payload KHÔNG group nào được nhận diện thì không phải report pub
    // outdated — lỗi schema, không phải "0 drift".
    assert!(parse_flutter_outdated_json(r#"{"foo": []}"#).is_err());
}

#[test]
fn flutter_outdated_malformed_entry_fails_not_silent_skip() {
    let raw = r#"{"packages": [
        {"package": "../evil", "current": "1.0.0", "latest": "2.0.0"},
        {"package": "ok", "current": "1.0.0", "latest": "2.0.0"}
    ]}"#;
    assert!(parse_flutter_outdated_json(raw).is_err());
}
