// Bare install adapter tests — exercise core selection and fail-closed errors.
// Test adapter install bare — kiểm tra chọn core và lỗi từ chối an toàn.
use super::*;

fn install_command() -> BareCommands {
    BareCommands::Install {
        packages: vec!["alpha".to_string()],
        frozen: true,
        ignore_scripts: true,
        allow_scripts: true,
        prefer_dedupe: true,
        repair: true,
        dry_run: true,
        offline: true,
        compat_runtime: Some("legacy".to_string()),
    }
}

#[test]
fn maps_all_nine_core_names_to_their_install_contract() {
    assert!(matches!(
        map(install_command(), Some("web")),
        Ok(CoreCommand::InstallWeb { .. })
    ));
    assert!(matches!(
        map(install_command(), Some("game")),
        Ok(CoreCommand::InstallGame { .. })
    ));
    assert!(matches!(
        map(install_command(), Some("ai")),
        Ok(CoreCommand::InstallAi { .. })
    ));
    assert!(matches!(
        map(install_command(), Some("clo")),
        Ok(CoreCommand::InstallClo { .. })
    ));
    assert!(matches!(
        map(install_command(), Some("cicd")),
        Ok(CoreCommand::InstallCicd { .. })
    ));
    assert!(matches!(
        map(install_command(), Some("iot")),
        Ok(CoreCommand::InstallIot { .. })
    ));
    assert!(matches!(
        map(install_command(), Some("app")),
        Ok(CoreCommand::InstallApp { .. })
    ));
    assert!(matches!(
        map(install_command(), Some("lib")),
        Ok(CoreCommand::InstallLib { .. })
    ));
    assert!(matches!(
        map(install_command(), Some("hardware")),
        Ok(CoreCommand::InstallHardware { .. })
    ));
}

#[test]
fn forwards_web_and_ai_specific_bare_flags() {
    assert!(matches!(
        map(install_command(), Some("web")),
        Ok(CoreCommand::InstallWeb {
            packages,
            frozen: true,
            ignore_scripts: true,
            allow_scripts: true,
            prefer_dedupe: true,
            repair: true,
            offline: true,
            compat_runtime: Some(runtime),
        }) if packages == vec!["alpha".to_string()] && runtime == "legacy"
    ));
    assert!(matches!(
        map(install_command(), Some("ai")),
        Ok(CoreCommand::InstallAi {
            packages,
            dry_run: true,
            frozen: true,
            offline: true,
            compat_runtime: Some(runtime),
        }) if packages == vec!["alpha".to_string()] && runtime == "legacy"
    ));
}

#[test]
fn preserves_unknown_core_diagnostic() {
    let error = map(install_command(), Some("unknown-core"))
        .err()
        .expect("unknown core must fail closed");

    assert!(error.to_string().contains("unknown-core"));
}

#[test]
fn preserves_missing_core_diagnostic() {
    let error = map(install_command(), None)
        .err()
        .expect("missing core must fail closed");

    assert!(error.to_string().contains("core"));
}

#[test]
fn rejects_a_command_routed_to_the_wrong_mapper() {
    let error = map(BareCommands::Docs { output: None }, Some("web"))
        .err()
        .expect("wrong command must fail closed");

    assert_eq!(
        error.to_string(),
        "bare install mapper received a different verb"
    );
}
