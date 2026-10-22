#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn debug_exposes_hosts_but_not_credentials() {
    let npmrc = NpmRc::parse(concat!(
        "registry=https://registry.example/\n",
        "//registry.example/:_authToken=super-secret-token\n",
        "//registry.example/:username=super-secret-user\n",
        "//registry.example/:_password=super-secret-password\n",
    ))
    .unwrap();
    let debug = format!("{npmrc:?}");
    assert!(debug.contains("registry.example"));
    assert!(debug.contains("auth_token_hosts"));
    assert!(!debug.contains("super-secret-token"));
    assert!(!debug.contains("super-secret-user"));
    assert!(!debug.contains("super-secret-password"));
}
