use mgc_types::{PackageId, Version};

/// A single resolution entry: pinned package + version + transitive deps.
#[derive(Debug, Clone)]
pub struct Resolution {
    pub package_id: PackageId,
    pub version: Version,
    pub integrity: String,
    pub deps: Vec<String>,
    pub dep_specs: Vec<(String, String)>,
    /// Names of peer dependencies (used for peer instance merge — 02 §2.1)
    pub peer_deps: Vec<String>,
}

/// Version selection preference for dedupe (02 §2.1 — Q8: default safe).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DedupePref {
    /// Pick the newest matching version (current behaviour, default).
    #[default]
    PreferLatest,
    /// Prefer a version already present in the graph/lockfile when it satisfies the range.
    PreferExisting,
}

/// Complete resolution result: all packages in topo-ish order.
#[derive(Debug, Clone)]
pub struct SolveResult {
    pub resolutions: Vec<Resolution>,
}
