// Project command mapping — preserve workspace query values.
// Mapping lệnh dự án — giữ giá trị truy vấn workspace.
use super::super::bare_common_to_common;
use crate::commands::definitions::commands::bare::BareCommands;
use crate::commands::workspace::WorkspaceCmd;
use crate::dispatch::types::CommonCommand;

#[test]
fn maps_workspace_filter_and_output_format() {
    assert!(matches!(
        bare_common_to_common(BareCommands::Workspace {
            cmd: WorkspaceCmd::List {
                filter: Some("./apps/*".to_owned()),
                json: true,
            },
        }),
        Some(CommonCommand::Workspace {
            cmd: WorkspaceCmd::List {
                filter: Some(filter),
                json: true,
            }
        }) if filter == "./apps/*"
    ));
}
