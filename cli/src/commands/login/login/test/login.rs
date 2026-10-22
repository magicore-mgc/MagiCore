//! Test pure registry selection for the login command.
//! Kiểm tra chọn registry thuần của lệnh login.
use super::{DEFAULT_REGISTRY, registry_or_default};

#[test]
fn configured_registry_is_preserved() {
    assert_eq!(
        registry_or_default(Some("https://registry.example/npm/".to_owned())),
        "https://registry.example/npm/"
    );
}

#[test]
fn absent_registry_uses_the_existing_default() {
    assert_eq!(registry_or_default(None), DEFAULT_REGISTRY);
}
