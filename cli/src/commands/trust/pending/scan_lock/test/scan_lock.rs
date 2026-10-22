//! Trust-pending behavior tests — test hành vi của trust pending.

use std::time::Duration;
#[test]
fn trust_scan_refuses_to_read_during_a_dependency_mutation() {
    let temp = tempfile::tempdir().expect("temporary project must be created");
    let _held =
        mgc_lockfile::project_lock::ProjectWriteLock::acquire(temp.path(), Duration::from_secs(1))
            .expect("first writer lock must be acquired");

    let result = super::acquire_scan_lock(temp.path(), Duration::from_millis(1));

    assert!(
        result.is_err(),
        "trust scan must not race an active dependency mutation"
    );
}
