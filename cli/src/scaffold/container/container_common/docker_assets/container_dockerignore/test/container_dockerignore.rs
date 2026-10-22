//! Tests the container dockerignore behavior.
//! Kiểm tra hành vi của owner container_dockerignore.

use super::super::container_dockerignore::container_dockerignore;
#[test]
fn dockerignore_excludes_magicore_state_and_native_build_outputs() {
    let dockerignore = container_dockerignore();
    for entry in [
        ".magicore/cache",
        ".magicore/store",
        "node_modules",
        "target",
    ] {
        assert!(dockerignore.lines().any(|line| line == entry));
    }
}
