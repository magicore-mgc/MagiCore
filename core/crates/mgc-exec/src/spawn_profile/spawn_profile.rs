use std::collections::HashSet;
use std::time::{Duration, Instant};

use sysinfo::System;

use super::process_invocation::observe_process;
use super::process_snapshot::refresh_parent_map;
use super::process_tree::subtree_pids;

/// Record every distinct `(basename, args)` pair in the live subtree of
/// `root_pid` for `duration`, sampling every `interval`.
/// Returns the sorted, deduplicated observation set. PM filtering happens at
/// policy time, not here, so this remains a faithful observation.
/// Trả tập quan sát đã sắp xếp và khử trùng. Lọc PM ở policy, không lọc lúc
/// capture, để giữ bản ghi đúng những gì đã quan sát.
pub fn record_subtree(
    root_pid: u32,
    duration: Duration,
    interval: Duration,
) -> Vec<(String, Vec<String>)> {
    let mut seen: HashSet<(String, Vec<String>)> = HashSet::new();
    let deadline = Instant::now() + duration;
    let mut system = System::new();
    while Instant::now() < deadline {
        let parent_of = refresh_parent_map(&mut system);
        let in_subtree = subtree_pids(root_pid, &parent_of);
        for pid in in_subtree {
            if let Some(observation) = observe_process(&system, pid) {
                seen.insert(observation);
            }
        }
        std::thread::sleep(interval);
    }
    let mut out: Vec<(String, Vec<String>)> = seen.into_iter().collect();
    out.sort();
    out
}
