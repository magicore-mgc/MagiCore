// Project command mapping — preserve template and workspace query values.
// Mapping lệnh dự án — giữ giá trị truy vấn template và workspace.
use super::super::bare_common_to_common;
use crate::commands::definitions::commands::bare::BareCommands;
use crate::commands::template::TemplateCmd;
use crate::commands::workspace::WorkspaceCmd;
use crate::dispatch::types::CommonCommand;

#[test]
fn maps_template_publish_all_subcommand() {
    assert!(matches!(
        bare_common_to_common(BareCommands::Template {
            cmd: TemplateCmd::PublishAll,
        }),
        Some(CommonCommand::Template {
            cmd: TemplateCmd::PublishAll
        })
    ));
}

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
