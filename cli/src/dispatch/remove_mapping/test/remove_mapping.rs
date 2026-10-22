// Shared remove projection tests — protect all core mappings and optional fields.
// Test phép chiếu remove dùng chung — giữ mapping và field tùy chọn của mọi core.
use super::*;

#[test]
fn recognizes_only_exact_core_names() {
    let routes = [
        ("web", RemoveTarget::Web),
        ("game", RemoveTarget::Game),
        ("ai", RemoveTarget::Ai),
        ("clo", RemoveTarget::Clo),
        ("cicd", RemoveTarget::Cicd),
        ("iot", RemoveTarget::Iot),
        ("app", RemoveTarget::App),
        ("lib", RemoveTarget::Lib),
        ("hardware", RemoveTarget::Hardware),
    ];

    for (name, expected) in routes {
        assert_eq!(RemoveTarget::from_core_name(name), Some(expected));
    }
    assert_eq!(RemoveTarget::from_core_name("Web"), None);
    assert_eq!(RemoveTarget::from_core_name("unknown"), None);
}

macro_rules! assert_remove_variant {
    ($target:expr, $variant:ident, $compat:expr) => {{
        let command = to_core_command(
            $target,
            RemoveRequest {
                packages: vec!["pkg-a".to_owned()],
                install_after_remove: true,
                compat_runtime: $compat,
            },
        );
        match command {
            CoreCommand::$variant {
                packages,
                compat_runtime,
                ..
            } => {
                assert_eq!(packages, vec!["pkg-a".to_owned()]);
                assert_eq!(compat_runtime, $compat);
            }
            _ => panic!("remove target produced a different core variant"),
        }
    }};
}

#[test]
fn maps_all_nine_targets_and_forwards_present_compat_runtime() {
    assert_remove_variant!(RemoveTarget::Web, RemoveWeb, Some("legacy".to_owned()));
    assert_remove_variant!(RemoveTarget::Game, RemoveGame, Some("legacy".to_owned()));
    assert_remove_variant!(RemoveTarget::Ai, RemoveAi, Some("legacy".to_owned()));
    assert_remove_variant!(RemoveTarget::Clo, RemoveClo, Some("legacy".to_owned()));
    assert_remove_variant!(RemoveTarget::Cicd, RemoveCicd, Some("legacy".to_owned()));
    assert_remove_variant!(RemoveTarget::Iot, RemoveIot, Some("legacy".to_owned()));
    assert_remove_variant!(RemoveTarget::App, RemoveApp, Some("legacy".to_owned()));
    assert_remove_variant!(RemoveTarget::Lib, RemoveLib, Some("legacy".to_owned()));
    assert_remove_variant!(
        RemoveTarget::Hardware,
        RemoveHardware,
        Some("legacy".to_owned())
    );
}

#[test]
fn maps_all_nine_targets_without_compat_runtime() {
    assert_remove_variant!(RemoveTarget::Web, RemoveWeb, None);
    assert_remove_variant!(RemoveTarget::Game, RemoveGame, None);
    assert_remove_variant!(RemoveTarget::Ai, RemoveAi, None);
    assert_remove_variant!(RemoveTarget::Clo, RemoveClo, None);
    assert_remove_variant!(RemoveTarget::Cicd, RemoveCicd, None);
    assert_remove_variant!(RemoveTarget::Iot, RemoveIot, None);
    assert_remove_variant!(RemoveTarget::App, RemoveApp, None);
    assert_remove_variant!(RemoveTarget::Lib, RemoveLib, None);
    assert_remove_variant!(RemoveTarget::Hardware, RemoveHardware, None);
}

#[test]
fn uses_requested_reinstall_value_for_web_only() {
    for (install_after_remove, expected) in [(true, true), (false, false)] {
        match to_core_command(
            RemoveTarget::Web,
            RemoveRequest {
                packages: vec!["pkg-a".to_owned()],
                install_after_remove,
                compat_runtime: None,
            },
        ) {
            CoreCommand::RemoveWeb { install, .. } => assert_eq!(install, expected),
            _ => panic!("web remove target produced a different core variant"),
        }
    }
}
