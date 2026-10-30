//! Behavior tests for `CoreKind`.
//! Test hành vi cho `CoreKind`.

use super::super::CoreKind;

#[test]
fn all_existing_core_names_round_trip() {
    let cases = [
        (CoreKind::Web, "web"),
        (CoreKind::Ai, "ai"),
        (CoreKind::App, "app"),
        (CoreKind::Lib, "lib"),
        (CoreKind::Game, "game"),
        (CoreKind::Iot, "iot"),
        (CoreKind::Cicd, "cicd"),
        // `clo` is the CLI identity (same value as `Ecosystem::as_str()`);
        // `cloud` is only the published template/registry namespace.
        // (`clo` là identity CLI; `cloud` chỉ là namespace template/registry.)
        (CoreKind::Cloud, "clo"),
        (CoreKind::Hardware, "hardware"),
    ];

    for (core, name) in cases {
        assert_eq!(core.as_str(), name);
        assert_eq!(CoreKind::from_str_core(name), Some(core));
    }
}

/// The layer namespace parses to the same kind but is never an identity.
/// (Namespace layer parse ra cùng kind nhưng không bao giờ là identity.)
#[test]
fn layer_namespace_parses_without_becoming_the_identity() {
    let cloud = CoreKind::from_str_core("cloud").expect("layer namespace names the core");
    assert_eq!(cloud, CoreKind::Cloud);
    assert_eq!(cloud.as_str(), "clo", "as_str() stays the CLI identity");
    assert_eq!(CoreKind::from_str_core("clo"), Some(CoreKind::Cloud));
}

#[test]
fn unknown_core_name_is_rejected() {
    assert_eq!(CoreKind::from_str_core("unknown"), None);
    assert_eq!(CoreKind::from_str_core("Web"), None);
}
