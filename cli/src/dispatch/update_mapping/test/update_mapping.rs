// Shared update projection tests — protect all core mappings and option fields.
// Test phép chiếu update dùng chung — giữ mapping và field tùy chọn của mọi core.
use super::*;

#[test]
fn recognizes_only_exact_core_names() {
    let routes = [
        ("web", UpdateTarget::Web),
        ("game", UpdateTarget::Game),
        ("ai", UpdateTarget::Ai),
        ("clo", UpdateTarget::Clo),
        ("cicd", UpdateTarget::Cicd),
        ("iot", UpdateTarget::Iot),
        ("app", UpdateTarget::App),
        ("lib", UpdateTarget::Lib),
        ("hardware", UpdateTarget::Hardware),
    ];

    for (name, expected) in routes {
        assert_eq!(UpdateTarget::from_core_name(name), Some(expected));
    }
    assert_eq!(UpdateTarget::from_core_name("Web"), None);
    assert_eq!(UpdateTarget::from_core_name("unknown"), None);
}

macro_rules! assert_update_variant {
    ($target:expr, $variant:ident, $install:expr, $compat:expr) => {{
        match to_core_command(
            $target,
            UpdateRequest {
                packages: vec!["pkg-a".to_owned()],
                install: $install,
                compat_runtime: $compat,
            },
        ) {
            CoreCommand::$variant {
                packages,
                install,
                compat_runtime,
            } => {
                assert_eq!(packages, vec!["pkg-a".to_owned()]);
                assert_eq!(install, $install);
                assert_eq!(compat_runtime, $compat);
            }
            _ => panic!("update target produced a different core variant"),
        }
    }};
}

#[test]
fn maps_all_nine_targets_with_present_compat_and_install_enabled() {
    assert_update_variant!(
        UpdateTarget::Web,
        UpdateWeb,
        true,
        Some("legacy".to_owned())
    );
    assert_update_variant!(
        UpdateTarget::Game,
        UpdateGame,
        true,
        Some("legacy".to_owned())
    );
    assert_update_variant!(UpdateTarget::Ai, UpdateAi, true, Some("legacy".to_owned()));
    assert_update_variant!(
        UpdateTarget::Clo,
        UpdateClo,
        true,
        Some("legacy".to_owned())
    );
    assert_update_variant!(
        UpdateTarget::Cicd,
        UpdateCicd,
        true,
        Some("legacy".to_owned())
    );
    assert_update_variant!(
        UpdateTarget::Iot,
        UpdateIot,
        true,
        Some("legacy".to_owned())
    );
    assert_update_variant!(
        UpdateTarget::App,
        UpdateApp,
        true,
        Some("legacy".to_owned())
    );
    assert_update_variant!(
        UpdateTarget::Lib,
        UpdateLib,
        true,
        Some("legacy".to_owned())
    );
    assert_update_variant!(
        UpdateTarget::Hardware,
        UpdateHardware,
        true,
        Some("legacy".to_owned())
    );
}

#[test]
fn maps_all_nine_targets_with_absent_compat_and_install_disabled() {
    assert_update_variant!(UpdateTarget::Web, UpdateWeb, false, None);
    assert_update_variant!(UpdateTarget::Game, UpdateGame, false, None);
    assert_update_variant!(UpdateTarget::Ai, UpdateAi, false, None);
    assert_update_variant!(UpdateTarget::Clo, UpdateClo, false, None);
    assert_update_variant!(UpdateTarget::Cicd, UpdateCicd, false, None);
    assert_update_variant!(UpdateTarget::Iot, UpdateIot, false, None);
    assert_update_variant!(UpdateTarget::App, UpdateApp, false, None);
    assert_update_variant!(UpdateTarget::Lib, UpdateLib, false, None);
    assert_update_variant!(UpdateTarget::Hardware, UpdateHardware, false, None);
}
