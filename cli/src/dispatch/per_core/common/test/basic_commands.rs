// Basic common command mapping — verify identity, search and canonical output.
// Mapping lệnh common cơ bản — xác minh identity, search và output chuẩn.
use super::super::common_to_dispatch;
use crate::commands::definitions::commands::common::CommonCommands;
use crate::dispatch::types::CommonCommand;

fn mapped(command: CommonCommands) -> CommonCommand {
    match common_to_dispatch(command) {
        Some(command) => command,
        None => panic!("common command must be mapped"),
    }
}

#[test]
fn maps_init_info_search_and_outdated_payloads() {
    match mapped(CommonCommands::Init {
        template: Some("web/react".to_owned()),
        signature: Some("core=web".to_owned()),
    }) {
        CommonCommand::Init {
            template,
            signature,
        } => {
            assert_eq!(template.as_deref(), Some("web/react"));
            assert_eq!(signature.as_deref(), Some("core=web"));
        }
        _ => panic!("init command mapped to a different variant"),
    }

    match mapped(CommonCommands::Info {
        package: "react".to_owned(),
        json: true,
    }) {
        CommonCommand::Info { package, json } => {
            assert_eq!(package, "react");
            assert!(json);
        }
        _ => panic!("info command mapped to a different variant"),
    }

    match mapped(CommonCommands::Search {
        query: "react".to_owned(),
        json: true,
        exact: true,
        page: Some(3),
    }) {
        CommonCommand::Search {
            query,
            json,
            exact,
            page,
        } => {
            assert_eq!(query, "react");
            assert!(json);
            assert!(exact);
            assert_eq!(page, Some(3));
        }
        _ => panic!("search command mapped to a different variant"),
    }

    match mapped(CommonCommands::Outdated { json: true }) {
        CommonCommand::Outdated { json } => assert!(json),
        _ => panic!("outdated command mapped to a different variant"),
    }
}

#[test]
fn maps_mcp_and_normalizes_both_capability_flag_values() {
    assert!(matches!(mapped(CommonCommands::Mcp), CommonCommand::Mcp));

    for json in [false, true] {
        assert!(matches!(
            mapped(CommonCommands::Capabilities { json }),
            CommonCommand::Capabilities
        ));
    }
}
