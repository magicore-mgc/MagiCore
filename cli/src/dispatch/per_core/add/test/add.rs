// Per-core add adapter tests — protect all explicit core command adapters.
// Test adapter add theo core — bảo vệ toàn bộ adapter lệnh tường minh.
use super::*;

fn packages() -> Vec<String> {
    vec!["alpha".to_string()]
}

#[test]
fn maps_all_nine_explicit_add_commands_and_supported_fields() {
    macro_rules! regular_command {
        ($variant:ident) => {
            AddCommands::$variant {
                packages: packages(),
                dev: true,
                exact: true,
                optional: true,
                peer: true,
                no_save: true,
                global: true,
                version: Some("1.2.3".to_string()),
                compat_runtime: Some("legacy".to_string()),
            }
        };
    }
    macro_rules! assert_regular_route {
        ($command:expr, $variant:ident) => {
            assert!(matches!(
                map($command),
                Some(CoreCommand::$variant {
                    packages: actual,
                    dev: true,
                    exact: true,
                    optional: true,
                    peer: true,
                    no_save: true,
                    global: true,
                    version: Some(version),
                    compat_runtime: Some(runtime),
                }) if actual == packages()
                    && version == "1.2.3"
                    && runtime == "legacy"
            ));
        };
    }

    assert!(matches!(
        map(AddCommands::AddWeb {
            packages: packages(),
            dev: true,
            exact: true,
            optional: true,
            peer: true,
            no_save: true,
            no_install: false,
            global: true,
            version: Some("1.2.3".to_string()),
            compat_runtime: Some("legacy".to_string()),
        }),
        Some(CoreCommand::AddWeb {
            packages: actual,
            dev: true,
            exact: true,
            optional: true,
            peer: true,
            no_save: true,
            install: true,
            global: true,
            version: Some(version),
            compat_runtime: Some(runtime),
        })
            if actual == packages() && version == "1.2.3" && runtime == "legacy"
    ));
    assert_regular_route!(regular_command!(AddGame), AddGame);
    assert_regular_route!(regular_command!(AddAi), AddAi);
    assert_regular_route!(regular_command!(AddClo), AddClo);
    assert_regular_route!(regular_command!(AddCicd), AddCicd);
    assert_regular_route!(regular_command!(AddIot), AddIot);
    assert_regular_route!(regular_command!(AddApp), AddApp);
    assert_regular_route!(regular_command!(AddLib), AddLib);
    assert!(matches!(
        map(AddCommands::AddHardware {
            packages: packages(),
            version: Some("1.2.3".to_string()),
            compat_runtime: Some("legacy".to_string()),
        }),
        Some(CoreCommand::AddHardware {
            packages: actual,
            version: Some(version),
            compat_runtime: Some(runtime),
        })
            if actual == packages() && version == "1.2.3" && runtime == "legacy"
    ));
}

#[test]
fn reverses_web_no_install_flag() {
    assert!(matches!(
        map(AddCommands::AddWeb {
            packages: packages(),
            dev: false,
            exact: false,
            optional: false,
            peer: false,
            no_save: false,
            no_install: true,
            global: false,
            version: None,
            compat_runtime: None,
        }),
        Some(CoreCommand::AddWeb { install: false, .. })
    ));
}
