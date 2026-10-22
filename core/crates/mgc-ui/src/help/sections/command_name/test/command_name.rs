//! Command Name help owner tests — test owner help command name.
//! Pin the nearest behavior contract without embedding test bodies in production files.
//! Giữ contract behavior gần owner, không đặt test body trong source production.

use super::per_core_name;

#[test]
fn command_name_uses_prefix_hyphen_core_spelling() {
    assert_eq!(per_core_name("install", "web"), "install-web");
    assert_eq!(per_core_name("create", "app"), "create-app");
}
