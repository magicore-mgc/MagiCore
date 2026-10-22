// Bare list adapter tests — exercise all-core selection and guarded errors.
// Test adapter list bare — kiểm tra chọn core và hàng rào lỗi.
use super::*;

fn list_command() -> BareCommands {
    BareCommands::List {
        compat_runtime: Some("legacy".to_string()),
    }
}

#[test]
fn maps_all_nine_core_names_and_forwards_compat_runtime() {
    macro_rules! assert_core_route {
        ($core:literal, $variant:ident) => {
            assert!(matches!(
                map(list_command(), Some($core)),
                Ok(CoreCommand::$variant {
                    compat_runtime: Some(value)
                }) if value == "legacy"
            ));
        };
    }

    assert_core_route!("web", ListWeb);
    assert_core_route!("game", ListGame);
    assert_core_route!("ai", ListAi);
    assert_core_route!("clo", ListClo);
    assert_core_route!("cicd", ListCicd);
    assert_core_route!("iot", ListIot);
    assert_core_route!("app", ListApp);
    assert_core_route!("lib", ListLib);
    assert_core_route!("hardware", ListHardware);
}

#[test]
fn preserves_missing_and_unknown_core_errors() {
    let missing = map(list_command(), None)
        .err()
        .expect("missing core must fail closed");
    assert!(missing.to_string().contains("core"));

    let unknown = map(list_command(), Some("unknown-core"))
        .err()
        .expect("unknown core must fail closed");
    assert!(unknown.to_string().contains("unknown-core"));
}

#[test]
fn rejects_a_command_routed_to_the_wrong_mapper() {
    let error = map(BareCommands::Docs { output: None }, Some("web"))
        .err()
        .expect("wrong command must fail closed");

    assert_eq!(
        error.to_string(),
        "bare list mapper received a different verb"
    );
}
