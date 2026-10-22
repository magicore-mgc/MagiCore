//! Debug redaction behavior test / test hành vi che bí mật Debug.

use crate::auth::Auth;

#[test]
fn debug_output_never_contains_token_or_password() {
    let auth = Auth {
        token: Some("SECRET-TOKEN".into()),
        username: Some("u".into()),
        password: Some("SECRET-PW".into()),
    };
    let debug = format!("{auth:?}");
    assert!(!debug.contains("SECRET-TOKEN"), "token leaked: {debug}");
    assert!(!debug.contains("SECRET-PW"), "password leaked: {debug}");
    assert!(
        debug.contains("[REDACTED]"),
        "redaction marker missing: {debug}"
    );
    assert!(debug.contains('u'), "username must stay visible: {debug}");
}
