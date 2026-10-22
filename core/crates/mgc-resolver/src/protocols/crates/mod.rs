//! Crates.io sparse-index protocol split by responsibility.
//! Protocol sparse-index Crates.io được chia theo trách nhiệm.

use super::{GRAPH_RESOLVE_CONCURRENCY, RegistryProtocol, RegistryRoot, ResolvedEntry};

const DEFAULT_INDEX_URL: &str = "https://index.crates.io";
const DEFAULT_DOWNLOAD_BASE: &str = "https://crates.io/api/v1/crates";

mod crates;
mod features;
mod http;
mod index;
mod materialize;
mod resolve;
mod types;
mod version;

pub use crates::CratesProtocol;
pub(super) use types::{
    ActivatedCrateFeatures, CrateDependencyRequest, CrateIndexEntry, CrateResolutionState,
};
#[cfg(test)]
pub(super) use version::{cargo_matches, sparse_prefix};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
