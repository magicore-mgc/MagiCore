use std::collections::{HashMap, HashSet};

use sysinfo::Pid;

/// Expand the root PID through the captured parent map until the subtree is closed.
/// Mở rộng PID gốc qua parent map đến khi tập subtree ổn định.
pub(in crate::spawn_profile) fn subtree_pids(
    root_pid: u32,
    parent_of: &HashMap<Pid, Pid>,
) -> HashSet<Pid> {
    let mut in_subtree = HashSet::from([Pid::from(root_pid as usize)]);
    let mut grown = true;
    while grown {
        grown = false;
        for (pid, parent) in parent_of {
            if in_subtree.contains(parent) && in_subtree.insert(*pid) {
                grown = true;
            }
        }
    }
    in_subtree
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
