//! Tests the write container entrypoint behavior.
//! Kiểm tra hành vi của owner write_container_entrypoint.

use super::super::write_container_entrypoint::write_container_entrypoint;
#[test]
fn entrypoint_replays_frozen_install_then_executes_mgc() {
    let project = tempfile::tempdir().unwrap();
    write_container_entrypoint(project.path(), "mgc install-web").unwrap();

    let entrypoint =
        std::fs::read_to_string(project.path().join("mgc-container-entrypoint")).unwrap();
    assert!(entrypoint.contains("mgc install-web --frozen"));
    assert!(entrypoint.ends_with("exec mgc \"$@\"\n"));
}
