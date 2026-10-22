// Explicit remove adapter tests — preserve all nine variant projections.
// Test adapter remove tường minh — giữ phép chiếu của đủ chín variant.
use super::*;

macro_rules! assert_explicit_variant {
    ($command:expr, $variant:ident) => {{
        let command = match map($command) {
            Some(command) => command,
            None => panic!("remove command must be handled"),
        };
        match command {
            CoreCommand::$variant {
                packages,
                compat_runtime,
                ..
            } => {
                assert_eq!(packages, vec!["pkg-a".to_owned()]);
                assert_eq!(compat_runtime.as_deref(), Some("legacy"));
            }
            _ => panic!("explicit remove variant produced a different core command"),
        }
    }};
}

#[test]
fn maps_all_nine_explicit_remove_variants_and_forwards_fields() {
    use RemoveCommands::*;

    assert_explicit_variant!(
        RemoveWeb {
            packages: vec!["pkg-a".to_owned()],
            no_install: false,
            compat_runtime: Some("legacy".to_owned()),
        },
        RemoveWeb
    );
    assert_explicit_variant!(
        RemoveGame {
            packages: vec!["pkg-a".to_owned()],
            compat_runtime: Some("legacy".to_owned()),
        },
        RemoveGame
    );
    assert_explicit_variant!(
        RemoveAi {
            packages: vec!["pkg-a".to_owned()],
            compat_runtime: Some("legacy".to_owned()),
        },
        RemoveAi
    );
    assert_explicit_variant!(
        RemoveClo {
            packages: vec!["pkg-a".to_owned()],
            compat_runtime: Some("legacy".to_owned()),
        },
        RemoveClo
    );
    assert_explicit_variant!(
        RemoveCicd {
            packages: vec!["pkg-a".to_owned()],
            compat_runtime: Some("legacy".to_owned()),
        },
        RemoveCicd
    );
    assert_explicit_variant!(
        RemoveIot {
            packages: vec!["pkg-a".to_owned()],
            compat_runtime: Some("legacy".to_owned()),
        },
        RemoveIot
    );
    assert_explicit_variant!(
        RemoveApp {
            packages: vec!["pkg-a".to_owned()],
            compat_runtime: Some("legacy".to_owned()),
        },
        RemoveApp
    );
    assert_explicit_variant!(
        RemoveLib {
            packages: vec!["pkg-a".to_owned()],
            compat_runtime: Some("legacy".to_owned()),
        },
        RemoveLib
    );
    assert_explicit_variant!(
        RemoveHardware {
            packages: vec!["pkg-a".to_owned()],
            compat_runtime: Some("legacy".to_owned()),
        },
        RemoveHardware
    );
}

#[test]
fn preserves_web_no_install_inversion() {
    for (no_install, expected_install) in [(false, true), (true, false)] {
        let command = match map(RemoveCommands::RemoveWeb {
            packages: vec!["pkg-a".to_owned()],
            no_install,
            compat_runtime: None,
        }) {
            Some(command) => command,
            None => panic!("Web remove must be handled"),
        };
        match command {
            CoreCommand::RemoveWeb { install, .. } => assert_eq!(install, expected_install),
            _ => panic!("Web remove mapping produced a different core command"),
        }
    }
}
