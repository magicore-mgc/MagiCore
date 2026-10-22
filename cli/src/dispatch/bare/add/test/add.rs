// Bare add adapter tests — exercise core selection and local error paths.
// Test adapter add bare — kiểm tra chọn core và các nhánh lỗi cục bộ.
use super::*;

fn add_command(no_install: bool) -> BareCommands {
    BareCommands::Add {
        packages: vec!["alpha".to_string()],
        version: Some("1.2.3".to_string()),
        dev: true,
        global: false,
        exact: true,
        optional: true,
        peer: true,
        no_save: true,
        no_install,
        compat_runtime: Some("legacy".to_string()),
    }
}

#[test]
fn maps_all_nine_core_names_to_their_add_contract() {
    assert!(matches!(
        map(add_command(false), Some("web")),
        Ok(CoreCommand::AddWeb { .. })
    ));
    assert!(matches!(
        map(add_command(false), Some("game")),
        Ok(CoreCommand::AddGame { .. })
    ));
    assert!(matches!(
        map(add_command(false), Some("ai")),
        Ok(CoreCommand::AddAi { .. })
    ));
    assert!(matches!(
        map(add_command(false), Some("clo")),
        Ok(CoreCommand::AddClo { .. })
    ));
    assert!(matches!(
        map(add_command(false), Some("cicd")),
        Ok(CoreCommand::AddCicd { .. })
    ));
    assert!(matches!(
        map(add_command(false), Some("iot")),
        Ok(CoreCommand::AddIot { .. })
    ));
    assert!(matches!(
        map(add_command(false), Some("app")),
        Ok(CoreCommand::AddApp { .. })
    ));
    assert!(matches!(
        map(add_command(false), Some("lib")),
        Ok(CoreCommand::AddLib { .. })
    ));
    assert!(matches!(
        map(add_command(false), Some("hardware")),
        Ok(CoreCommand::AddHardware { .. })
    ));
}

#[test]
fn preserves_web_no_install_inversion_and_version() {
    assert!(matches!(
        map(add_command(false), Some("web")),
        Ok(CoreCommand::AddWeb { install: true, version: Some(version), .. }) if version == "1.2.3"
    ));
    assert!(matches!(
        map(add_command(true), Some("web")),
        Ok(CoreCommand::AddWeb { install: false, version: Some(version), .. }) if version == "1.2.3"
    ));
}

#[test]
fn preserves_unknown_and_missing_core_errors() {
    let unknown = map(add_command(false), Some("unknown-core"))
        .err()
        .expect("unknown core must fail closed");
    assert!(unknown.to_string().contains("unknown-core"));

    let missing = map(add_command(false), None)
        .err()
        .expect("missing core must fail closed");
    assert!(missing.to_string().contains("core"));
}

#[test]
fn rejects_a_command_routed_to_the_wrong_mapper() {
    let error = map(BareCommands::Docs { output: None }, Some("web"))
        .err()
        .expect("wrong command must fail closed");

    assert_eq!(
        error.to_string(),
        "bare add mapper received a different verb"
    );
}
