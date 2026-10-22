//! Test registration request body projection.
//! Kiểm tra phép chiếu body request đăng ký.
#![allow(clippy::unwrap_used)]

use super::registration_body;

#[test]
fn registration_body_keeps_credentials_and_adds_caller_fields() {
    let body = registration_body(
        "alice",
        "secret",
        &serde_json::json!({ "email": "", "role": "publisher" }),
    );
    assert_eq!(body["name"], "alice");
    assert_eq!(body["password"], "secret");
    assert_eq!(body["role"], "publisher");
}
