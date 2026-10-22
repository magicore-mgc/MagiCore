//! Dependency Closure owner.
//! Phụ trách dependency closure của native engine.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use mgc_resolver::protocols::ResolvedEntry;
use mgc_types::MgResult;

use crate::native::engine::engine::entry_index::resolve_entry_key;

pub(in crate::native::engine::engine) fn runtime_reachable(
    runtime_roots: HashSet<String>,
    chosen: &HashMap<String, ResolvedEntry>,
    by_name: &BTreeMap<String, Vec<String>>,
) -> MgResult<HashSet<String>> {
    let mut runtime_reachable = HashSet::new();
    let mut runtime_queue = runtime_roots.into_iter().collect::<VecDeque<_>>();
    while let Some(key) = runtime_queue.pop_front() {
        if !runtime_reachable.insert(key.clone()) {
            continue;
        }
        if let Some(entry) = chosen.get(&key) {
            for (dependency, range) in &entry.deps {
                runtime_queue.push_back(resolve_entry_key(dependency, range, chosen, by_name)?);
            }
        }
    }
    Ok(runtime_reachable)
}
