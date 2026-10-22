// Per-core install adapter tests — protect all nine CLI-to-core routes.
// Test adapter install theo core — bảo vệ đủ chín route CLI sang core.
use super::*;

fn packages() -> Vec<String> {
    vec!["alpha".to_string()]
}

#[test]
fn maps_all_nine_explicit_install_commands() {
    assert!(matches!(
        map(InstallCommands::InstallWeb {
            packages: packages(),
            frozen: true,
            ignore_scripts: true,
            allow_scripts: true,
            prefer_dedupe: true,
            repair: true,
            offline: true,
            compat_runtime: Some("legacy".to_string()),
        }),
        Some(CoreCommand::InstallWeb {
            frozen: true,
            ignore_scripts: true,
            allow_scripts: true,
            prefer_dedupe: true,
            repair: true,
            offline: true,
            compat_runtime: Some(value),
            ..
        }) if value == "legacy"
    ));
    assert!(matches!(
        map(InstallCommands::InstallGame {
            packages: packages(),
            frozen: true,
            compat_runtime: Some("legacy".to_string()),
        }),
        Some(CoreCommand::InstallGame {
            frozen: true,
            compat_runtime: Some(value),
            ..
        }) if value == "legacy"
    ));
    assert!(matches!(
        map(InstallCommands::InstallAi {
            packages: packages(),
            dry_run: true,
            compat_runtime: Some("legacy".to_string()),
            frozen: true,
            offline: true,
        }),
        Some(CoreCommand::InstallAi {
            dry_run: true,
            frozen: true,
            offline: true,
            compat_runtime: Some(value),
            ..
        }) if value == "legacy"
    ));
    assert!(matches!(
        map(InstallCommands::InstallClo {
            packages: packages(),
            frozen: true,
            compat_runtime: Some("legacy".to_string()),
        }),
        Some(CoreCommand::InstallClo {
            dry_run: false,
            frozen: true,
            compat_runtime: Some(value),
            ..
        }) if value == "legacy"
    ));
    assert!(matches!(
        map(InstallCommands::InstallCicd {
            packages: packages(),
            compat_runtime: Some("legacy".to_string()),
        }),
        Some(CoreCommand::InstallCicd {
            dry_run: false,
            compat_runtime: Some(value),
            ..
        }) if value == "legacy"
    ));
    assert!(matches!(
        map(InstallCommands::InstallIot {
            packages: packages(),
            frozen: true,
            compat_runtime: Some("legacy".to_string()),
        }),
        Some(CoreCommand::InstallIot {
            frozen: true,
            compat_runtime: Some(value),
            ..
        }) if value == "legacy"
    ));
    assert!(matches!(
        map(InstallCommands::InstallApp {
            packages: packages(),
            compat_runtime: Some("legacy".to_string()),
            frozen: true,
            offline: true,
        }),
        Some(CoreCommand::InstallApp {
            dry_run: false,
            frozen: true,
            offline: true,
            compat_runtime: Some(value),
            ..
        }) if value == "legacy"
    ));
    assert!(matches!(
        map(InstallCommands::InstallLib {
            packages: packages(),
            compat_runtime: Some("legacy".to_string()),
            frozen: true,
            offline: true,
        }),
        Some(CoreCommand::InstallLib {
            frozen: true,
            offline: true,
            compat_runtime: Some(value),
            ..
        }) if value == "legacy"
    ));
    assert!(matches!(
        map(InstallCommands::InstallHardware {
            packages: packages(),
            compat_runtime: Some("legacy".to_string()),
        }),
        Some(CoreCommand::InstallHardware {
            compat_runtime: Some(value),
            ..
        }) if value == "legacy"
    ));
}
