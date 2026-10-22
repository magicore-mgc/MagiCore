// Per-core list adapter tests — protect all explicit list command routes.
// Test adapter list theo core — bảo vệ đủ route list tường minh.
use super::*;

#[test]
fn maps_all_nine_explicit_list_commands_and_forwards_compat_runtime() {
    macro_rules! assert_list_route {
        ($command:ident, $variant:ident) => {
            assert!(matches!(
                map(ListCommands::$command {
                    compat_runtime: Some("legacy".to_string()),
                }),
                Some(CoreCommand::$variant {
                    compat_runtime: Some(value)
                }) if value == "legacy"
            ));
        };
    }

    assert_list_route!(ListWeb, ListWeb);
    assert_list_route!(ListGame, ListGame);
    assert_list_route!(ListAi, ListAi);
    assert_list_route!(ListClo, ListClo);
    assert_list_route!(ListCicd, ListCicd);
    assert_list_route!(ListIot, ListIot);
    assert_list_route!(ListApp, ListApp);
    assert_list_route!(ListLib, ListLib);
    assert_list_route!(ListHardware, ListHardware);
}
