//! Authorization header behavior tests / test hành vi Authorization.

use crate::auth::Auth;

#[test]
fn header_prefers_bearer() {
    let auth = Auth {
        token: Some("t".into()),
        username: Some("u".into()),
        password: Some("p".into()),
    };
    assert_eq!(auth.header_value().as_deref(), Some("Bearer t"));
}

#[test]
fn header_basic_when_no_token() {
    let auth = Auth {
        token: None,
        username: Some("u".into()),
        password: Some("pass".into()),
    };
    assert_eq!(auth.header_value().as_deref(), Some("Basic dTpwYXNz"));
}

#[test]
fn header_none_when_no_auth() {
    assert_eq!(Auth::default().header_value(), None);
}
