//! Help help owner tests — test owner help help.
//! Pin the nearest behavior contract without embedding test bodies in production files.
//! Giữ contract behavior gần owner, không đặt test body trong source production.

use super::custom_help_text;

#[test]
fn web_single_core_keeps_shared_lifecycle_commands() {
    let output = custom_help_text(&[("web", "Web")]);

    assert!(output.contains("Build: single-core (web)"));
    assert!(output.contains("Scaffold a new web project"));
    assert!(output.contains("Add a dependency"));
    assert!(output.contains("Remove a dependency"));
    assert!(output.contains("Update packages"));
    assert!(output.contains("List installed packages"));
    assert!(!output.contains("add-web"));
}

#[test]
fn non_web_single_core_uses_scoped_create_and_package_commands() {
    let output = custom_help_text(&[("app", "App")]);

    assert!(output.contains("Build: single-core (app)"));
    assert!(output.contains("create-app"));
    assert!(output.contains("add-app"));
    assert!(output.contains("remove-app"));
    assert!(output.contains("update-app"));
    assert!(output.contains("list-app"));
    assert!(!output.contains("Scaffold a new web project"));
}

#[test]
fn multiple_cores_keep_named_create_and_lifecycle_commands() {
    let output = custom_help_text(&[("web", "Web"), ("app", "App")]);

    assert!(output.contains("Build: multi-core (web, app)"));
    assert!(output.contains("create-web"));
    assert!(output.contains("create-app"));
    assert!(output.contains("add-web"));
    assert!(output.contains("add-app"));
}

#[test]
fn empty_core_list_does_not_invent_a_target_core() {
    let output = custom_help_text(&[]);

    assert!(output.contains("Build: multi-core ()"));
    assert!(output.contains("Target core ()"));
    assert!(!output.contains("install-"));
    assert!(!output.contains("create-"));
}
