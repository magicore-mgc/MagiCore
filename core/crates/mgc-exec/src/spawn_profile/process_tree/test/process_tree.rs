use std::collections::HashMap;

use sysinfo::Pid;

use super::super::subtree_pids;

#[test]
fn expands_multiple_generations_and_ignores_other_roots() {
    let root = Pid::from(10_usize);
    let child = Pid::from(11_usize);
    let grandchild = Pid::from(12_usize);
    let unrelated = Pid::from(30_usize);
    let parents = HashMap::from([
        (child, root),
        (grandchild, child),
        (unrelated, Pid::from(31_usize)),
    ]);

    let observed = subtree_pids(10, &parents);
    assert!(observed.contains(&root));
    assert!(observed.contains(&child));
    assert!(observed.contains(&grandchild));
    assert!(!observed.contains(&unrelated));
}
