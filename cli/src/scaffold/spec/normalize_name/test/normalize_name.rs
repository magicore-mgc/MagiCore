//! Behavior tests for template-name normalization.
//! Test hành vi chuẩn hóa tên template.

use super::super::normalize_template_name;

#[test]
fn lowercases_replaces_underscores_and_trims() {
    assert_eq!(normalize_template_name(" React_Native "), "react-native");
}

#[test]
fn preserves_other_existing_characters() {
    assert_eq!(normalize_template_name("My  Framework"), "my  framework");
}
