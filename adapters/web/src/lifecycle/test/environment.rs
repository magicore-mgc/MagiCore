//! Lifecycle behavior tests for the matching production owner.
//! Test hành vi lifecycle cho owner production tương ứng.

use super::super::environment::lifecycle_path_env;

#[test]
fn lifecycle_path_env_prepends_node_modules_bin_when_present() {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("node_modules").join(".bin");
    std::fs::create_dir_all(&bin).unwrap();

    let path_env = lifecycle_path_env(dir.path()).unwrap();
    let paths: Vec<_> = std::env::split_paths(&path_env).collect();

    assert_eq!(paths.first(), Some(&bin));
}
