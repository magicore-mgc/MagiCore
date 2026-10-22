//! Crates.io sparse-index rows and feature-closure state.
//! Kiểu dòng sparse-index Crates.io và trạng thái đóng feature.

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

/// One crate version entry from the sparse index NDJSON.
/// Một entry version crate từ NDJSON sparse-index.
#[derive(Debug, Clone, Deserialize)]
pub(in crate::protocols) struct CrateIndexEntry {
    pub(in crate::protocols::crates) name: String,
    pub(in crate::protocols::crates) vers: String,
    #[serde(default)]
    pub(in crate::protocols::crates) deps: Vec<CrateDep>,
    #[serde(default)]
    pub(in crate::protocols::crates) features: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub(in crate::protocols::crates) features2: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub(in crate::protocols::crates) cksum: Option<String>,
    #[serde(default)]
    pub(in crate::protocols::crates) yanked: bool,
}

/// One dependency edge in a crate version entry.
/// Một cạnh dependency trong entry version crate.
#[derive(Debug, Clone, Deserialize)]
pub(in crate::protocols) struct CrateDep {
    pub(in crate::protocols::crates) name: String,
    #[serde(default)]
    pub(in crate::protocols::crates) req: String,
    #[serde(default)]
    pub(in crate::protocols::crates) features: Vec<String>,
    #[serde(default = "default_true")]
    pub(in crate::protocols::crates) default_features: bool,
    #[serde(default)]
    pub(in crate::protocols::crates) kind: Option<String>,
    #[serde(default)]
    pub(in crate::protocols::crates) optional: bool,
    #[serde(default)]
    pub(in crate::protocols::crates) target: Option<String>,
    #[serde(default)]
    pub(in crate::protocols::crates) package: Option<String>,
    #[serde(default)]
    pub(in crate::protocols::crates) registry: Option<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Default)]
pub(in crate::protocols) struct CrateDependencyRequest {
    pub(in crate::protocols::crates) ranges: Vec<String>,
    pub(in crate::protocols::crates) features: BTreeSet<String>,
    pub(in crate::protocols::crates) default_features: bool,
}

pub(in crate::protocols) struct CrateResolutionState {
    pub(in crate::protocols::crates) entry: CrateIndexEntry,
    pub(in crate::protocols::crates) requested_features: BTreeSet<String>,
    pub(in crate::protocols::crates) default_features: bool,
    /// Whether requested features activate this package, or it is retained
    /// only because an active feature weakly references its optional edge.
    /// Package active theo feature, hoặc giữ lại vì weak optional edge.
    pub(in crate::protocols::crates) active: bool,
    pub(in crate::protocols::crates) resolved_dependencies: BTreeSet<(String, String)>,
    pub(in crate::protocols::crates) extra_markers: BTreeSet<String>,
}

#[derive(Default)]
pub(in crate::protocols) struct ActivatedCrateFeatures {
    pub(in crate::protocols::crates) optional_dependencies: BTreeSet<String>,
    pub(in crate::protocols::crates) lock_only_optional_dependencies: BTreeSet<String>,
    pub(in crate::protocols::crates) dependency_features: BTreeMap<String, BTreeSet<String>>,
}
