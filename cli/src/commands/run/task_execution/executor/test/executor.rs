//! Local task binary resolution without host-tool allowlist expansion.
//! Phân giải binary task trong project, không mở rộng quyền tool host.

use super::*;

#[test]
fn resolves_only_owned_package_binaries_and_leaves_missing_tools_to_host_policy() {
    let project = tempfile::tempdir().expect("temporary project");
    let bin = project.path().join("node_modules/.bin");
    std::fs::create_dir_all(&bin).expect("create binary directory");
    #[cfg(windows)]
    let binary = bin.join("vitest.cmd");
    #[cfg(not(windows))]
    let binary = bin.join("vitest");
    std::fs::write(&binary, "fixture").expect("write owned binary");

    assert_eq!(
        resolve_task_binary("vitest", Some(&bin), project.path()).expect("resolve tool"),
        Some(binary)
    );
    assert!(
        resolve_task_binary("missing-tool", Some(&bin), project.path())
            .expect("missing tool")
            .is_none()
    );
    assert!(
        resolve_task_binary("../vitest", Some(&bin), project.path())
            .expect("explicit path remains guarded downstream")
            .is_none()
    );
    assert!(
        resolve_task_binary("vitest", None, project.path())
            .expect("no package bin context")
            .is_none()
    );
}
