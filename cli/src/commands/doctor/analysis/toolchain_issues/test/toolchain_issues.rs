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
            required_by_core: None,
        },
        Tool {
            name: "python3",
            path: None,
            version: "MISSING".into(),
            required_by_core: None,
        },
        Tool {
            name: "git",
            path: None,
            version: "MISSING".into(),
            required_by_core: None,
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
        required_by_core: None,
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

#[test]
fn missing_core_tool_is_a_failure_that_names_the_core_and_the_tool() {
    // A core the project actually uses reports its missing tool as CRITICAL so
    // `mgc doctor` becomes a real readiness gate.
    // Core project đang dùng báo tool thiếu ở mức CRITICAL để `mgc doctor`
    // thành cổng sẵn sàng thật.
    let tools = vec![Tool {
        name: "flutter",
        path: None,
        version: "MISSING".into(),
        required_by_core: Some("app"),
    }];

    let (issues, actions) = diagnostics(&tools);

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].code, "ERR_CORE_TOOLCHAIN_MISSING");
    assert_eq!(issues[0].severity, "CRITICAL");
    assert!(issues[0].message.contains("'app'"));
    assert!(issues[0].message.contains("'flutter'"));
    assert_eq!(actions.len(), 1);
    assert!(actions[0].contains("flutter"));
    assert!(actions[0].contains("app"));
}

#[test]
fn present_core_tool_is_not_reported_as_missing() {
    let tools = vec![Tool {
        name: "node",
        path: Some("/usr/bin/node".into()),
        version: "v22".into(),
        required_by_core: Some("web"),
    }];

    let (issues, _actions) = diagnostics(&tools);

    assert!(issues.is_empty());
}
