use sysinfo::System;

use super::super::refresh_parent_map;

#[test]
fn snapshot_contains_only_processes_with_a_parent() {
    let mut system = System::new();
    let parents = refresh_parent_map(&mut system);
    assert!(parents.iter().all(|(child, parent)| child != parent));
}
