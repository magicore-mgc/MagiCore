// Bare remove adapter tests — protect all core routes, option projection and errors.
// Test adapter remove bare — giữ route core, phép chiếu cờ và lỗi.
use super::*;

fn bare_remove(no_install: bool, compat_runtime: Option<&str>) -> BareCommands {
    BareCommands::Remove {
        packages: vec!["pkg-a".to_owned()],
        no_install,
        compat_runtime: compat_runtime.map(String::from),
    }
}

macro_rules! assert_bare_variant {
    ($core:expr, $variant:ident) => {{
        match map(bare_remove(false, Some("legacy")), Some($core)).expect("valid bare remove") {
            CoreCommand::$variant {
                packages,
                compat_runtime,
                ..
            } => {
                assert_eq!(packages, vec!["pkg-a".to_owned()]);
                assert_eq!(compat_runtime.as_deref(), Some("legacy"));
            }
            _ => panic!("bare remove core name produced a different variant"),
        }
    }};
}

#[test]
fn routes_all_nine_core_names_and_forwards_package_and_compat_values() {
    assert_bare_variant!("web", RemoveWeb);
    assert_bare_variant!("game", RemoveGame);
    assert_bare_variant!("ai", RemoveAi);
    assert_bare_variant!("clo", RemoveClo);
    assert_bare_variant!("cicd", RemoveCicd);
    assert_bare_variant!("iot", RemoveIot);
    assert_bare_variant!("app", RemoveApp);
    assert_bare_variant!("lib", RemoveLib);
    assert_bare_variant!("hardware", RemoveHardware);
}

#[test]
fn preserves_missing_and_unknown_core_diagnostics() {
    let missing = map(bare_remove(false, None), None)
        .err()
        .expect("missing core must fail");
    assert!(missing.to_string().contains("core"));

    let unknown = map(bare_remove(false, None), Some("unknown"))
        .err()
        .expect("unknown core must fail");
    assert!(unknown.to_string().contains("unknown"));
}

#[test]
fn projects_no_install_only_to_the_web_reinstall_field() {
    for (no_install, expected_install) in [(false, true), (true, false)] {
        match map(bare_remove(no_install, None), Some("web")).expect("valid Web remove") {
            CoreCommand::RemoveWeb { install, .. } => assert_eq!(install, expected_install),
            _ => panic!("Web remove mapping produced a different variant"),
        }
    }
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
        "bare remove mapper received a different verb"
    );
    assert!(error.downcast_ref::<BareRemoveError>().is_some());
}
