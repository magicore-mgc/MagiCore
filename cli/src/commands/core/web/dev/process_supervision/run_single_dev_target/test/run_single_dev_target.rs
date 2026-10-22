use super::super::super::super::super::types::DevTarget;
use super::super::run_single_dev_target;
use crate::commands::compat::CompatMode;

#[tokio::test]
async fn missing_native_entrypoint_fails_before_launch() {
    let dir = tempfile::tempdir().expect("temporary project directory");
    let target = DevTarget {
        dir: dir.path().to_path_buf(),
        role: "root",
        script_name: "dev",
        host: None,
        port: None,
    };

    let result = run_single_dev_target(&target, &CompatMode::Native, None).await;

    assert!(result.is_err());
    assert!(!dir.path().join(".magicore/exec.log").exists());
}
