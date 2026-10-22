//! Dependency Closure source regression test.
//! Test hồi quy source cho dependency closure.

use std::collections::HashSet;

use mgc_resolver::protocols::ResolvedEntry;

use crate::native::engine::engine::dependency_closure::runtime_reachable;
use crate::native::engine::engine::entry_index::index_entries;

fn entry(name: &str, deps: Vec<(&str, &str)>) -> ResolvedEntry {
    ResolvedEntry {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        deps: deps
            .into_iter()
            .map(|(name, range)| (name.to_string(), range.to_string()))
            .collect(),
        artifact_url: format!("https://registry.invalid/{name}"),
        sha256: String::new(),
        extra_markers: Vec::new(),
    }
}

#[test]
fn runtime_reachability_includes_transitives_but_not_another_dev_root() {
    let entries = vec![
        entry("app", vec![("shared", "^1.0")]),
        entry("shared", Vec::new()),
        entry("test-tool", vec![("shared", "^1.0")]),
    ];
    let (chosen, by_name) = index_entries(&entries).unwrap();
    let roots = HashSet::from(["app@1.0.0".to_string()]);

    let reachable = runtime_reachable(roots, &chosen, &by_name).unwrap();

    assert!(reachable.contains("app@1.0.0"));
    assert!(reachable.contains("shared@1.0.0"));
    assert!(!reachable.contains("test-tool@1.0.0"));
}
