//! Wires resolved-registry package entry construction.
//! Wiring tạo entry package registry đã resolve.

mod error;
mod registry_entries;

use mgc_types::{MgError, MgResult, ResolvedGraph};
use std::collections::BTreeSet;
use std::path::Path;

use registry_entries::build_registry_entries_inner;

pub(super) fn build_registry_entries(
    graph: &ResolvedGraph,
    pub_cache: &Path,
    seen: &mut BTreeSet<String>,
) -> MgResult<Vec<super::super::types::FlutterPackageConfigEntry>> {
    build_registry_entries_inner(graph, pub_cache, seen).map_err(MgError::from)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
