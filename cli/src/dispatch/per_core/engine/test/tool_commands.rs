// Tool command mapping — retain package, executable and runtime arguments.
// Mapping lệnh tool — giữ package, executable và đối số runtime.
use super::super::engine_to_common;
use crate::commands::definitions::ToolCmd;
use crate::commands::definitions::commands::engine::EngineCommands;
use crate::dispatch::types::CommonCommand;

#[test]
fn maps_dlx_package_binary_lifecycle_flag_and_arguments() {
    match engine_to_common(EngineCommands::Dlx {
        package: "@openai/codex@1.0.0".to_owned(),
        bin: Some("codex".to_owned()),
        allow_scripts: true,
        args: vec!["--version".to_owned()],
    }) {
        Some(CommonCommand::Dlx {
            package,
            bin,
            allow_scripts,
            args,
        }) => {
            assert_eq!(package, "@openai/codex@1.0.0");
            assert_eq!(bin.as_deref(), Some("codex"));
            assert!(allow_scripts);
            assert_eq!(args, vec!["--version".to_owned()]);
        }
        _ => panic!("dlx command or payload was changed"),
    }
}

#[test]
fn maps_persistent_tool_install_options() {
    match engine_to_common(EngineCommands::Tool {
        cmd: ToolCmd::Install {
            package: "npm:@example/tool@latest".to_owned(),
            bin: Some("example".to_owned()),
            allow_scripts: false,
        },
    }) {
        Some(CommonCommand::Tool {
            cmd:
                ToolCmd::Install {
                    package,
                    bin,
                    allow_scripts,
                },
        }) => {
            assert_eq!(package, "npm:@example/tool@latest");
            assert_eq!(bin.as_deref(), Some("example"));
            assert!(!allow_scripts);
        }
        _ => panic!("tool command or payload was changed"),
    }
}
