// Shared install mapping tests — verify target parsing and core-specific fields.
// Test map install dùng chung — kiểm tra nhận diện target và trường riêng từng core.
use super::*;

fn request() -> InstallRequest {
    InstallRequest {
        packages: vec!["alpha".to_string()],
        dry_run: true,
        frozen: true,
        ignore_scripts: true,
        allow_scripts: true,
        prefer_dedupe: true,
        repair: true,
        offline: true,
        compat_runtime: Some("legacy".to_string()),
    }
}

#[test]
fn recognizes_only_the_existing_lowercase_core_names() {
    assert_eq!(
        InstallTarget::from_core_name("web"),
        Some(InstallTarget::Web)
    );
    assert_eq!(
        InstallTarget::from_core_name("game"),
        Some(InstallTarget::Game)
    );
    assert_eq!(InstallTarget::from_core_name("ai"), Some(InstallTarget::Ai));
    assert_eq!(
        InstallTarget::from_core_name("clo"),
        Some(InstallTarget::Clo)
    );
    assert_eq!(
        InstallTarget::from_core_name("cicd"),
        Some(InstallTarget::Cicd)
    );
    assert_eq!(
        InstallTarget::from_core_name("iot"),
        Some(InstallTarget::Iot)
    );
    assert_eq!(
        InstallTarget::from_core_name("app"),
        Some(InstallTarget::App)
    );
    assert_eq!(
        InstallTarget::from_core_name("lib"),
        Some(InstallTarget::Lib)
    );
    assert_eq!(
        InstallTarget::from_core_name("hardware"),
        Some(InstallTarget::Hardware)
    );
    assert_eq!(InstallTarget::from_core_name("Web"), None);
    assert_eq!(InstallTarget::from_core_name("unknown"), None);
}

#[test]
fn maps_web_install_options_without_loss() {
    assert!(matches!(
        to_core_command(InstallTarget::Web, request()),
        CoreCommand::InstallWeb {
            packages,
            frozen: true,
            ignore_scripts: true,
            allow_scripts: true,
            prefer_dedupe: true,
            repair: true,
            offline: true,
            compat_runtime: Some(value),
        } if packages == vec!["alpha".to_string()] && value == "legacy"
    ));
}

#[test]
fn maps_the_remaining_eight_core_contracts() {
    assert!(matches!(
        to_core_command(InstallTarget::Game, request()),
        CoreCommand::InstallGame { packages, frozen: true, compat_runtime: Some(value) }
            if packages == vec!["alpha".to_string()] && value == "legacy"
    ));
    assert!(matches!(
        to_core_command(InstallTarget::Ai, request()),
        CoreCommand::InstallAi { packages, dry_run: true, frozen: true, offline: true, compat_runtime: Some(value) }
            if packages == vec!["alpha".to_string()] && value == "legacy"
    ));
    assert!(matches!(
        to_core_command(InstallTarget::Clo, request()),
        CoreCommand::InstallClo { packages, dry_run: true, frozen: true, compat_runtime: Some(value) }
            if packages == vec!["alpha".to_string()] && value == "legacy"
    ));
    assert!(matches!(
        to_core_command(InstallTarget::Cicd, request()),
        CoreCommand::InstallCicd { packages, dry_run: true, compat_runtime: Some(value) }
            if packages == vec!["alpha".to_string()] && value == "legacy"
    ));
    assert!(matches!(
        to_core_command(InstallTarget::Iot, request()),
        CoreCommand::InstallIot { packages, frozen: true, compat_runtime: Some(value) }
            if packages == vec!["alpha".to_string()] && value == "legacy"
    ));
    assert!(matches!(
        to_core_command(InstallTarget::App, request()),
        CoreCommand::InstallApp { packages, dry_run: true, frozen: true, offline: true, compat_runtime: Some(value) }
            if packages == vec!["alpha".to_string()] && value == "legacy"
    ));
    assert!(matches!(
        to_core_command(InstallTarget::Lib, request()),
        CoreCommand::InstallLib { packages, frozen: true, offline: true, compat_runtime: Some(value) }
            if packages == vec!["alpha".to_string()] && value == "legacy"
    ));
    assert!(matches!(
        to_core_command(InstallTarget::Hardware, request()),
        CoreCommand::InstallHardware { packages, compat_runtime: Some(value) }
            if packages == vec!["alpha".to_string()] && value == "legacy"
    ));
}
