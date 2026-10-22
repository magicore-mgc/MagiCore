//! Entry Index owner.
//! Phụ trách entry index của native engine.

use std::collections::{BTreeMap, HashMap};

use mgc_resolver::protocols::ResolvedEntry;
use mgc_types::MgResult;

pub(in crate::native::engine::engine) type EntryIndex = (
    HashMap<String, ResolvedEntry>,
    BTreeMap<String, Vec<String>>,
);

pub(in crate::native::engine::engine) fn index_entries(
    entries: &[ResolvedEntry],
) -> MgResult<EntryIndex> {
    let chosen = entries
        .iter()
        .map(|entry| (entry_key(&entry.name, &entry.version), entry.clone()))
        .collect::<HashMap<String, ResolvedEntry>>();
    let mut by_name = BTreeMap::<String, Vec<String>>::new();
    for key in chosen.keys() {
        let (name, _) = key.rsplit_once('@').ok_or_else(|| {
            mgc_types::MgError::Other("resolved package identity is malformed".to_string())
        })?;
        by_name
            .entry(name.to_string())
            .or_default()
            .push(key.clone());
    }
    Ok((chosen, by_name))
}

pub(in crate::native::engine::engine) fn entry_key(name: &str, version: &str) -> String {
    format!("{name}@{version}")
}

pub(in crate::native::engine::engine) fn resolve_entry_key(
    name: &str,
    range: &str,
    entries: &HashMap<String, ResolvedEntry>,
    by_name: &BTreeMap<String, Vec<String>>,
) -> MgResult<String> {
    if let Some(version) = range.strip_prefix('=') {
        let key = entry_key(name, version);
        if entries.contains_key(&key) {
            return Ok(key);
        }
    }
    match by_name.get(name).map(Vec::as_slice).unwrap_or_default() {
        [key] => Ok(key.clone()),
        [] => Err(mgc_types::MgError::DependencyConflict(format!(
            "resolved graph is missing dependency '{name}' ({range})"
        ))),
        _ => Err(mgc_types::MgError::DependencyConflict(format!(
            "resolved dependency '{name}' is ambiguous without an exact version in range '{range}'"
        ))),
    }
}
