//! Tests for `DlxInvocation` preparation.

use super::super::DlxInvocation;

#[test]
fn invocation_preserves_package_options_arguments_and_caller_directory() {
    let expected_cwd = std::env::current_dir().unwrap();
    let invocation = DlxInvocation::prepare(
        "@example/tool@1.2.3".to_string(),
        Some("tool-bin".to_string()),
        true,
        vec!["--help".to_string(), "value".to_string()],
    )
    .unwrap();

    assert_eq!(invocation.package, "@example/tool@1.2.3");
    assert_eq!(invocation.selected_bin.as_deref(), Some("tool-bin"));
    assert!(invocation.allow_scripts);
    assert_eq!(
        invocation.args,
        vec!["--help".to_string(), "value".to_string()]
    );
    assert_eq!(invocation.execution_cwd, expected_cwd);
    assert_eq!(
        invocation.purpose,
        crate::commands::package_runtime::PackageRunPurpose::Dlx
    );
}
