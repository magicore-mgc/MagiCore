//! Existing behavior test for this cache-prune owner.
//! Test hành vi có sẵn cho owner cache-prune này.

use super::support::tempdir_real;
use crate::cache_prune::prune_unlinked_old_cas_files_under;

#[test]
fn test_project_cas_prune_keeps_hardlinked_live_blobs() {
    let temp = tempdir_real().unwrap();
    let cas = temp.path().join("cas");
    let blob = cas.join("ab").join("live");
    let orphan = cas.join("cd").join("orphan");
    let live_link = temp.path().join("node_modules").join("live");
    std::fs::create_dir_all(blob.parent().unwrap()).unwrap();
    std::fs::create_dir_all(orphan.parent().unwrap()).unwrap();
    std::fs::create_dir_all(live_link.parent().unwrap()).unwrap();
    std::fs::write(&blob, b"live").unwrap();
    std::fs::write(&orphan, b"orphan").unwrap();
    std::fs::hard_link(&blob, &live_link).unwrap();

    prune_unlinked_old_cas_files_under(&cas, std::time::Duration::from_secs(0)).unwrap();

    assert!(blob.exists());
    assert!(live_link.exists());
    assert!(!orphan.exists());
}
