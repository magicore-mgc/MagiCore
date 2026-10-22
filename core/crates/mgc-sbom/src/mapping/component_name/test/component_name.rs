#![allow(clippy::unwrap_used)]
//! Component name mapping tests — Kiểm thử ánh xạ tên component

use super::super::*;

#[test]
fn swift_component_name_uses_the_repository_leaf() {
    assert_eq!(
        component_name(
            EcosystemTag::Swift,
            "https://git.example.invalid/org/library.git",
        ),
        "library",
    );
}
