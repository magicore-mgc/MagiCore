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
        (CoreKind::Cloud, "cloud"),
        (CoreKind::Hardware, "hardware"),
    ];

    for (core, name) in cases {
        assert_eq!(core.as_str(), name);
        assert_eq!(CoreKind::from_str_core(name), Some(core));
    }
}

#[test]
fn unknown_core_name_is_rejected() {
    assert_eq!(CoreKind::from_str_core("unknown"), None);
    assert_eq!(CoreKind::from_str_core("Web"), None);
}
