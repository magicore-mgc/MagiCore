//! Preserves required-tool selection and issue/action order.
//! Giữ lựa chọn tool bắt buộc và thứ tự issue/action.

use super::super::diagnostics;
use crate::commands::doctor::Tool;

#[test]
fn warns_for_missing_git_and_node_but_not_optional_inventory_tools() {
    let tools = vec![
        Tool {
            name: "node",
            path: None,
            version: "MISSING".into(),
        },
        Tool {
            name: "python3",
            path: None,
            version: "MISSING".into(),
        },
        Tool {
            name: "git",
            path: None,
            version: "MISSING".into(),
        },
    ];
    let (issues, actions) = diagnostics(&tools);

    assert_eq!(issues.len(), 2);
    assert_eq!(
        issues[0].message,
        "Core tool 'node' is not installed or not in PATH."
    );
    assert_eq!(
        issues[1].message,
        "Core tool 'git' is not installed or not in PATH."
    );
    assert_eq!(actions, vec!["Please install node", "Please install git"]);
}

#[test]
fn warns_when_a_found_tool_has_no_guarded_version_result() {
    let tools = vec![Tool {
        name: "docker",
        path: Some("/usr/bin/docker".into()),
        version: "?".into(),
    }];

    let (issues, actions) = diagnostics(&tools);

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].code, "WARN_TOOLCHAIN_VERSION_UNAVAILABLE");
    assert_eq!(issues[0].severity, "WARNING");
    assert!(issues[0].message.contains("docker"));
    assert_eq!(issues[0].fix_command, None);
    assert_eq!(
        actions,
        vec!["Check that 'docker' responds to its fixed version probe."]
    );
}
