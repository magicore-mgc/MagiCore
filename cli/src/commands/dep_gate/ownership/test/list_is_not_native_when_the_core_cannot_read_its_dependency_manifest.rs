//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn list_is_not_native_when_the_core_cannot_read_its_dependency_manifest() {
    for (core, ecosystem, framework) in [
        ("game", Some("godot"), Some("godot")),
        ("game", Some("unity"), Some("unity")),
        ("game", Some("unreal"), Some("unreal")),
        ("iot", Some("zephyr"), Some("zephyr")),
        ("clo", Some("terraform"), Some("terraform")),
        ("clo", Some("cloudflare"), Some("cloudflare")),
    ] {
        let context = DepContext::new(core, ecosystem, framework, None, DepOp::List);
        assert!(
            matches!(owner_for(&context), DepOwner::Unsupported),
            "{core}/{ecosystem:?} has no native package list implementation"
        );
    }
}
