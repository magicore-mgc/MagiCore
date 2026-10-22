//! Coordinator helper tests.

use super::super::default_host;

#[test]
fn dev_host_defaults_to_localhost() {
    assert_eq!(default_host(None), "localhost");
}

#[test]
fn dev_keeps_explicit_host() {
    assert_eq!(default_host(Some("0.0.0.0".to_string())), "0.0.0.0");
}
