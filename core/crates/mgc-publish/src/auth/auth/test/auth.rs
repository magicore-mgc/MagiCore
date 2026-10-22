//! Auth value behavior tests / test hành vi kiểu Auth.

use crate::auth::Auth;

#[test]
fn default_auth_is_empty() {
    assert!(Auth::default().is_empty());
}

#[test]
fn any_credential_makes_auth_non_empty() {
    assert!(
        !Auth {
            token: Some("token".into()),
            ..Auth::default()
        }
        .is_empty()
    );
}
