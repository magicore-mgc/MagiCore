use super::super::super::super::super::types::{DevLaunch, DevTarget};
use super::super::run_dev_launch_with_guard;
use crate::commands::compat::CompatMode;

#[test]
fn missing_program_returns_launch_error() {
    let dir = tempfile::tempdir().expect("temporary project directory");
    let target = DevTarget {
        dir: dir.path().to_path_buf(),
        role: "root",
        script_name: "dev",
        host: None,
        port: None,
    };
    let launch = DevLaunch {
        program: dir.path().join("missing-mgc-dev-command"),
        args: Vec::new(),
        envs: Vec::new(),
    };

    let result = run_dev_launch_with_guard(&target, &launch, &CompatMode::Native, None, None);

    assert!(result.is_err());
}
