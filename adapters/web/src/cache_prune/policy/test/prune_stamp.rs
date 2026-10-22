//! Existing behavior test for this cache-prune owner.
//! Test hành vi có sẵn cho owner cache-prune này.

use crate::cache_prune::{
    project_cache_prune_due, shared_cache_prune_due, write_project_cache_prune_stamp,
};

#[test]
fn prune_stamp_skips_redundant_cache_walks() {
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(project_cache_prune_due(temp.path()));

    write_project_cache_prune_stamp(temp.path()).expect("write prune stamp");

    assert!(!project_cache_prune_due(temp.path()));
    assert!(shared_cache_prune_due(temp.path()));
}
