//! Remove-operation regression tests. / Regression test thao tác remove.

use super::support::project;
use super::*;
use std::time::Duration;

#[tokio::test]
async fn patch_remove_refuses_to_race_install() {
    let temp = tempfile::tempdir().unwrap();
    project(temp.path());
    let before = std::fs::read(temp.path().join("mgc.toml")).unwrap();
    let _held =
        mgc_lockfile::project_lock::ProjectWriteLock::acquire(temp.path(), Duration::from_secs(1))
            .unwrap();

    let result = run_at_project_root(
        PatchArgs {
            cmd: PatchCmd::Remove {
                package: "example".to_string(),
            },
        },
        temp.path(),
    )
    .await;

    assert!(result.is_err(), "patch remove must not race install");
    assert_eq!(std::fs::read(temp.path().join("mgc.toml")).unwrap(), before);
}
