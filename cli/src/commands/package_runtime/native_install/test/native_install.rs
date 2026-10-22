//! Focused native install regression tests.
//! Test hồi quy tập trung cho native install.

use super::*;

#[test]
fn native_install_invocation_keeps_mgc_project_and_install_scope() {
    let directory = tempfile::tempdir().expect("runner project");
    let workspace = RunnerWorkspace {
        _directory: directory,
        project_root: std::env::current_dir().expect("current directory"),
        canonical_project: std::env::current_dir().expect("canonical current directory"),
    };
    let invocation = create_invocation(&workspace, false).expect("build native install invocation");
    assert_eq!(
        invocation.binary,
        std::env::current_exe().expect("current MGC executable")
    );
    assert_eq!(
        invocation.args,
        ["install", "--core", "web", "--ignore-scripts"]
    );
    assert_eq!(
        invocation.options.cwd.as_deref(),
        Some(workspace.project_root.as_path())
    );
    assert!(!invocation.options.clean_env);
    assert!(matches!(
        invocation.options.execution_scope,
        Some(mgc_exec::allowlist::ExecutionScope::Install)
    ));
}

#[test]
fn native_install_invocation_preserves_explicit_script_opt_in() {
    let directory = tempfile::tempdir().expect("runner project");
    let workspace = RunnerWorkspace {
        _directory: directory,
        project_root: std::env::current_dir().expect("current directory"),
        canonical_project: std::env::current_dir().expect("canonical current directory"),
    };
    let invocation = create_invocation(&workspace, true).expect("build opt-in install invocation");
    assert_eq!(
        invocation.args,
        ["install", "--core", "web", "--allow-scripts"]
    );
}
