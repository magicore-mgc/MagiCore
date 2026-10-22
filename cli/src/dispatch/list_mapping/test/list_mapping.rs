// Shared list mapping tests — verify exact targets and compatibility forwarding.
// Test map list dùng chung — kiểm tra target chính xác và chuyển cờ tương thích.
use super::*;

fn request() -> ListRequest {
    ListRequest {
        compat_runtime: Some("legacy".to_string()),
    }
}

fn request_without_compat_runtime() -> ListRequest {
    ListRequest {
        compat_runtime: None,
    }
}

#[test]
fn recognizes_only_existing_lowercase_core_names() {
    assert_eq!(ListTarget::from_core_name("web"), Some(ListTarget::Web));
    assert_eq!(ListTarget::from_core_name("game"), Some(ListTarget::Game));
    assert_eq!(ListTarget::from_core_name("ai"), Some(ListTarget::Ai));
    assert_eq!(ListTarget::from_core_name("clo"), Some(ListTarget::Clo));
    assert_eq!(ListTarget::from_core_name("cicd"), Some(ListTarget::Cicd));
    assert_eq!(ListTarget::from_core_name("iot"), Some(ListTarget::Iot));
    assert_eq!(ListTarget::from_core_name("app"), Some(ListTarget::App));
    assert_eq!(ListTarget::from_core_name("lib"), Some(ListTarget::Lib));
    assert_eq!(
        ListTarget::from_core_name("hardware"),
        Some(ListTarget::Hardware)
    );
    assert_eq!(ListTarget::from_core_name("Web"), None);
    assert_eq!(ListTarget::from_core_name("unknown"), None);
}

#[test]
fn forwards_compat_runtime_for_all_nine_core_contracts() {
    macro_rules! assert_forwarded {
        ($target:expr, $variant:ident) => {
            assert!(matches!(
                to_core_command($target, request()),
                CoreCommand::$variant {
                    compat_runtime: Some(value)
                } if value == "legacy"
            ));
        };
    }

    assert_forwarded!(ListTarget::Web, ListWeb);
    assert_forwarded!(ListTarget::Game, ListGame);
    assert_forwarded!(ListTarget::Ai, ListAi);
    assert_forwarded!(ListTarget::Clo, ListClo);
    assert_forwarded!(ListTarget::Cicd, ListCicd);
    assert_forwarded!(ListTarget::Iot, ListIot);
    assert_forwarded!(ListTarget::App, ListApp);
    assert_forwarded!(ListTarget::Lib, ListLib);
    assert_forwarded!(ListTarget::Hardware, ListHardware);
}

#[test]
fn does_not_invent_compat_runtime_for_any_core() {
    macro_rules! assert_absent {
        ($target:expr, $variant:ident) => {
            assert!(matches!(
                to_core_command($target, request_without_compat_runtime()),
                CoreCommand::$variant {
                    compat_runtime: None
                }
            ));
        };
    }

    assert_absent!(ListTarget::Web, ListWeb);
    assert_absent!(ListTarget::Game, ListGame);
    assert_absent!(ListTarget::Ai, ListAi);
    assert_absent!(ListTarget::Clo, ListClo);
    assert_absent!(ListTarget::Cicd, ListCicd);
    assert_absent!(ListTarget::Iot, ListIot);
    assert_absent!(ListTarget::App, ListApp);
    assert_absent!(ListTarget::Lib, ListLib);
    assert_absent!(ListTarget::Hardware, ListHardware);
}
