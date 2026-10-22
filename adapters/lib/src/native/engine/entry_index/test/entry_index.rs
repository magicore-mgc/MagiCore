//! Entry Index source regression test.
//! Test hồi quy source cho entry index.

use std::collections::HashMap;

use mgc_resolver::protocols::ResolvedEntry;

use crate::native::engine::engine::entry_index::{index_entries, resolve_entry_key};

fn entry(version: &str) -> ResolvedEntry {
    ResolvedEntry {
        name: "shared".to_string(),
        version: version.to_string(),
        deps: Vec::new(),
        artifact_url: "https://registry.invalid/shared".to_string(),
        sha256: String::new(),
        extra_markers: Vec::new(),
    }
}

#[test]
fn exact_versions_disambiguate_and_unpinned_multiple_versions_fail_closed() {
    let entries = vec![entry("1.0.0"), entry("2.0.0")];
    let (chosen, by_name) = index_entries(&entries).unwrap();

    assert_eq!(
        resolve_entry_key("shared", "=2.0.0", &chosen, &by_name).unwrap(),
        "shared@2.0.0"
    );
    let missing = HashMap::new();
    assert!(resolve_entry_key("missing", "^1.0", &missing, &by_name).is_err());
    assert!(resolve_entry_key("shared", "^1.0", &chosen, &by_name).is_err());
}
