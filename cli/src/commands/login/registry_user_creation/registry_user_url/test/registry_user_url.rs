//! Test registry user URL construction and policy.
//! Kiểm tra tạo URL user registry và policy.
#![allow(clippy::unwrap_used)]

use super::registry_user_url;

#[test]
fn registry_user_url_preserves_prefix_and_encodes_username() {
    let url = registry_user_url("https://registry.example/npm/team/", "alice/admin").unwrap();
    assert_eq!(
        url.as_str(),
        "https://registry.example/npm/team/-/user/org.couchdb.user:alice%2Fadmin"
    );
}

#[test]
fn registry_user_url_rejects_insecure_remote_and_credential_urls() {
    assert!(registry_user_url("http://registry.example/npm", "alice").is_err());
    assert!(registry_user_url("https://user:pass@registry.example/npm", "alice").is_err());
    assert!(registry_user_url("https://registry.example/npm?token=x", "alice").is_err());
}

#[test]
fn loopback_http_registry_remains_available_for_local_development() {
    assert!(registry_user_url("http://127.0.0.1:4315/npm", "alice").is_ok());
    assert!(registry_user_url("http://[::1]:4315/npm", "alice").is_ok());
}
