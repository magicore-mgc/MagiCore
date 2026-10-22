//! Tests for Game dev execution configuration.

use std::path::Path;

use super::super::execution_options;

#[test]
fn game_dev_uses_project_scoped_clean_execution() {
    let root = Path::new("/tmp/mgc-game-project");
    let options = execution_options(root);
    let expected_log = root.join(".magicore").join("exec.log");

    assert_eq!(options.cwd.as_deref(), Some(root));
    assert_eq!(options.log_path.as_ref(), Some(&expected_log));
    assert!(options.clean_env);
    assert!(matches!(
        options.execution_scope,
        Some(mgc_exec::allowlist::ExecutionScope::DevServer)
    ));
}
