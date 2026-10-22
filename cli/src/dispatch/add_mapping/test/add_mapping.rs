// Shared add mapping tests — verify target names and all core projections.
// Test map add dùng chung — kiểm tra tên target và phép chiếu đủ core.
use super::*;

fn request() -> AddRequest {
    AddRequest {
        packages: vec!["alpha".to_string()],
        dev: true,
        exact: true,
        optional: true,
        peer: true,
        no_save: true,
        install: true,
        global: true,
        compat_runtime: Some("legacy".to_string()),
        version: Some("1.2.3".to_string()),
    }
}

#[test]
fn recognizes_only_the_existing_lowercase_core_names() {
    assert_eq!(AddTarget::from_core_name("web"), Some(AddTarget::Web));
    assert_eq!(AddTarget::from_core_name("game"), Some(AddTarget::Game));
    assert_eq!(AddTarget::from_core_name("ai"), Some(AddTarget::Ai));
    assert_eq!(AddTarget::from_core_name("clo"), Some(AddTarget::Clo));
    assert_eq!(AddTarget::from_core_name("cicd"), Some(AddTarget::Cicd));
    assert_eq!(AddTarget::from_core_name("iot"), Some(AddTarget::Iot));
    assert_eq!(AddTarget::from_core_name("app"), Some(AddTarget::App));
    assert_eq!(AddTarget::from_core_name("lib"), Some(AddTarget::Lib));
    assert_eq!(
        AddTarget::from_core_name("hardware"),
        Some(AddTarget::Hardware)
    );
    assert_eq!(AddTarget::from_core_name("Web"), None);
    assert_eq!(AddTarget::from_core_name("unknown"), None);
}

#[test]
fn maps_web_with_the_full_dependency_option_contract() {
    assert!(matches!(
        to_core_command(AddTarget::Web, request()),
        CoreCommand::AddWeb {
            packages,
            dev: true,
            exact: true,
            optional: true,
            peer: true,
            no_save: true,
            install: true,
            global: true,
            compat_runtime: Some(runtime),
            version: Some(version),
        } if packages == vec!["alpha".to_string()]
            && runtime == "legacy"
            && version == "1.2.3"
    ));
}

#[test]
fn maps_all_non_web_dependency_core_contracts() {
    macro_rules! assert_regular_core {
        ($target:expr, $variant:ident) => {
            assert!(matches!(
                to_core_command($target, request()),
                CoreCommand::$variant {
                    packages,
                    dev: true,
                    exact: true,
                    optional: true,
                    peer: true,
                    no_save: true,
                    global: true,
                    compat_runtime: Some(runtime),
                    version: Some(version),
                } if packages == vec!["alpha".to_string()]
                    && runtime == "legacy"
                    && version == "1.2.3"
            ));
        };
    }

    assert_regular_core!(AddTarget::Game, AddGame);
    assert_regular_core!(AddTarget::Ai, AddAi);
    assert_regular_core!(AddTarget::Clo, AddClo);
    assert_regular_core!(AddTarget::Cicd, AddCicd);
    assert_regular_core!(AddTarget::Iot, AddIot);
    assert_regular_core!(AddTarget::App, AddApp);
    assert_regular_core!(AddTarget::Lib, AddLib);
}

#[test]
fn keeps_hardware_add_contract_narrow() {
    assert!(matches!(
        to_core_command(AddTarget::Hardware, request()),
        CoreCommand::AddHardware {
            packages,
            compat_runtime: Some(runtime),
            version: Some(version),
        } if packages == vec!["alpha".to_string()]
            && runtime == "legacy"
            && version == "1.2.3"
    ));
}
