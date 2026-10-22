// Explicit update adapter tests — preserve all nine field projections.
// Test adapter update tường minh — giữ phép chiếu field của đủ chín core.
use super::*;

macro_rules! assert_explicit_update {
    ($command:expr, $variant:ident, $install:expr, $compat:expr) => {{
        let command = match map($command) {
            Some(command) => command,
            None => panic!("update command must be handled"),
        };
        match command {
            CoreCommand::$variant {
                packages,
                install,
                compat_runtime,
            } => {
                assert_eq!(packages, vec!["pkg-a".to_owned()]);
                assert_eq!(install, $install);
                assert_eq!(compat_runtime.as_deref(), $compat);
            }
            _ => panic!("explicit update variant produced a different core command"),
        }
    }};
}

#[test]
fn maps_all_nine_explicit_variants_with_present_compatibility() {
    use UpdateCommands::*;

    assert_explicit_update!(
        UpdateWeb {
            packages: vec!["pkg-a".to_owned()],
            install: true,
            compat_runtime: Some("legacy".to_owned())
        },
        UpdateWeb,
        true,
        Some("legacy")
    );
    assert_explicit_update!(
        UpdateGame {
            packages: vec!["pkg-a".to_owned()],
            install: true,
            compat_runtime: Some("legacy".to_owned())
        },
        UpdateGame,
        true,
        Some("legacy")
    );
    assert_explicit_update!(
        UpdateAi {
            packages: vec!["pkg-a".to_owned()],
            install: true,
            compat_runtime: Some("legacy".to_owned())
        },
        UpdateAi,
        true,
        Some("legacy")
    );
    assert_explicit_update!(
        UpdateClo {
            packages: vec!["pkg-a".to_owned()],
            install: true,
            compat_runtime: Some("legacy".to_owned())
        },
        UpdateClo,
        true,
        Some("legacy")
    );
    assert_explicit_update!(
        UpdateCicd {
            packages: vec!["pkg-a".to_owned()],
            install: true,
            compat_runtime: Some("legacy".to_owned())
        },
        UpdateCicd,
        true,
        Some("legacy")
    );
    assert_explicit_update!(
        UpdateIot {
            packages: vec!["pkg-a".to_owned()],
            install: true,
            compat_runtime: Some("legacy".to_owned())
        },
        UpdateIot,
        true,
        Some("legacy")
    );
    assert_explicit_update!(
        UpdateApp {
            packages: vec!["pkg-a".to_owned()],
            install: true,
            compat_runtime: Some("legacy".to_owned())
        },
        UpdateApp,
        true,
        Some("legacy")
    );
    assert_explicit_update!(
        UpdateLib {
            packages: vec!["pkg-a".to_owned()],
            install: true,
            compat_runtime: Some("legacy".to_owned())
        },
        UpdateLib,
        true,
        Some("legacy")
    );
    assert_explicit_update!(
        UpdateHardware {
            packages: vec!["pkg-a".to_owned()],
            install: true,
            compat_runtime: Some("legacy".to_owned())
        },
        UpdateHardware,
        true,
        Some("legacy")
    );
}

#[test]
fn maps_all_nine_explicit_variants_with_install_disabled_and_no_compatibility() {
    use UpdateCommands::*;

    assert_explicit_update!(
        UpdateWeb {
            packages: vec!["pkg-a".to_owned()],
            install: false,
            compat_runtime: None,
        },
        UpdateWeb,
        false,
        None
    );
    assert_explicit_update!(
        UpdateGame {
            packages: vec!["pkg-a".to_owned()],
            install: false,
            compat_runtime: None,
        },
        UpdateGame,
        false,
        None
    );
    assert_explicit_update!(
        UpdateAi {
            packages: vec!["pkg-a".to_owned()],
            install: false,
            compat_runtime: None,
        },
        UpdateAi,
        false,
        None
    );
    assert_explicit_update!(
        UpdateClo {
            packages: vec!["pkg-a".to_owned()],
            install: false,
            compat_runtime: None,
        },
        UpdateClo,
        false,
        None
    );
    assert_explicit_update!(
        UpdateCicd {
            packages: vec!["pkg-a".to_owned()],
            install: false,
            compat_runtime: None,
        },
        UpdateCicd,
        false,
        None
    );
    assert_explicit_update!(
        UpdateIot {
            packages: vec!["pkg-a".to_owned()],
            install: false,
            compat_runtime: None,
        },
        UpdateIot,
        false,
        None
    );
    assert_explicit_update!(
        UpdateApp {
            packages: vec!["pkg-a".to_owned()],
            install: false,
            compat_runtime: None,
        },
        UpdateApp,
        false,
        None
    );
    assert_explicit_update!(
        UpdateLib {
            packages: vec!["pkg-a".to_owned()],
            install: false,
            compat_runtime: None,
        },
        UpdateLib,
        false,
        None
    );
    assert_explicit_update!(
        UpdateHardware {
            packages: vec!["pkg-a".to_owned()],
            install: false,
            compat_runtime: None,
        },
        UpdateHardware,
        false,
        None
    );
}
