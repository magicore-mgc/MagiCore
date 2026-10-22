//! Wires the package configuration owner.
//! Wiring owner package configuration.

mod error;
mod package_config;
mod registry_entries;
mod sdk_entries;

use mgc_types::{MgError, MgResult, ResolvedGraph};
use std::path::Path;

use package_config::write_flutter_package_config_with_sdk_root_inner;

pub(in crate::install) fn write_flutter_package_config_with_sdk_root(
    graph: &ResolvedGraph,
    project_root: &Path,
    pub_cache: &Path,
    sdk_root: Option<&Path>,
) -> MgResult<()> {
    write_flutter_package_config_with_sdk_root_inner(graph, project_root, pub_cache, sdk_root)
        .map_err(MgError::from)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
