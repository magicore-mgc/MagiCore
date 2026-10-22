//! Registry URL policy tests.
//! Test policy URL registry.

use super::*;

#[test]
fn validate_https_required() {
    assert!(validate_registry_url("https://registry.example.com", false).is_ok());
    assert!(validate_registry_url("http://localhost:4315", true).is_ok());
    assert!(validate_registry_url("http://127.0.0.1:4315", true).is_ok());
    assert!(validate_registry_url("http://remote.com", false).is_err());
}
