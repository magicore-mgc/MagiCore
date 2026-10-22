// Bare update adapter tests — protect all routes, field values and diagnostics.
// Test adapter update bare — giữ route, giá trị field và diagnostic.
use super::*;

fn bare_update(install: bool, compat_runtime: Option<&str>) -> BareCommands {
    BareCommands::Update {
        packages: vec!["pkg-a".to_owned()],
        install,
        compat_runtime: compat_runtime.map(String::from),
    }
}

macro_rules! assert_bare_update_variant {
    ($core:expr, $variant:ident, $install:expr, $compat:expr) => {{
        match map(bare_update($install, $compat), Some($core)).expect("valid bare update") {
            CoreCommand::$variant {
                packages,
                install,
                compat_runtime,
            } => {
                assert_eq!(packages, vec!["pkg-a".to_owned()]);
                assert_eq!(install, $install);
                assert_eq!(compat_runtime.as_deref(), $compat);
            }
            _ => panic!("bare update core name produced a different variant"),
        }
    }};
}

#[test]
fn routes_all_nine_core_names_and_forwards_present_values() {
    assert_bare_update_variant!("web", UpdateWeb, true, Some("legacy"));
    assert_bare_update_variant!("game", UpdateGame, true, Some("legacy"));
    assert_bare_update_variant!("ai", UpdateAi, true, Some("legacy"));
    assert_bare_update_variant!("clo", UpdateClo, true, Some("legacy"));
    assert_bare_update_variant!("cicd", UpdateCicd, true, Some("legacy"));
    assert_bare_update_variant!("iot", UpdateIot, true, Some("legacy"));
    assert_bare_update_variant!("app", UpdateApp, true, Some("legacy"));
    assert_bare_update_variant!("lib", UpdateLib, true, Some("legacy"));
    assert_bare_update_variant!("hardware", UpdateHardware, true, Some("legacy"));
}

#[test]
fn routes_all_nine_core_names_with_install_disabled_and_no_compat() {
    assert_bare_update_variant!("web", UpdateWeb, false, None);
    assert_bare_update_variant!("game", UpdateGame, false, None);
    assert_bare_update_variant!("ai", UpdateAi, false, None);
    assert_bare_update_variant!("clo", UpdateClo, false, None);
    assert_bare_update_variant!("cicd", UpdateCicd, false, None);
    assert_bare_update_variant!("iot", UpdateIot, false, None);
    assert_bare_update_variant!("app", UpdateApp, false, None);
    assert_bare_update_variant!("lib", UpdateLib, false, None);
    assert_bare_update_variant!("hardware", UpdateHardware, false, None);
}

#[test]
fn preserves_missing_and_unknown_core_diagnostics() {
    let missing = map(bare_update(true, None), None)
        .err()
        .expect("missing core must fail");
    assert!(missing.to_string().contains("core"));

    let unknown = map(bare_update(true, None), Some("unknown"))
        .err()
        .expect("unknown core must fail");
    assert!(unknown.to_string().contains("unknown"));
}

#[test]
fn wrong_verb_returns_the_local_typed_error() {
    let error = map(
        BareCommands::List {
            compat_runtime: None,
        },
        Some("web"),
    )
    .err()
    .expect("wrong verb must fail");
    assert_eq!(
        error.to_string(),
        "bare update mapper received a different verb"
    );
    assert!(error.downcast_ref::<BareUpdateError>().is_some());
}
