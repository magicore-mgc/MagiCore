//! Cache command and statistics types.
//! Định nghĩa kiểu action, target, entry và thống kê cache.

use anyhow::{Result, bail};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::commands::cache) enum CacheTarget {
    All,
    Shared,
    Project,
    Build,
}

impl CacheTarget {
    pub(in crate::commands::cache) fn parse(raw: &str) -> Result<Self> {
        match raw {
            "all" => Ok(Self::All),
            "shared" => Ok(Self::Shared),
            "project" => Ok(Self::Project),
            "build" => Ok(Self::Build),
            other => bail!("unknown cache target: {other}"),
        }
    }

    pub(in crate::commands::cache) fn includes(self, target: Self) -> bool {
        self == Self::All || self == target
    }
}

#[derive(Debug)]
pub(in crate::commands::cache) struct CacheEntry {
    pub(in crate::commands::cache) label: &'static str,
    pub(in crate::commands::cache) path: PathBuf,
    pub(in crate::commands::cache) removable: bool,
}

#[derive(Debug, Default)]
pub(in crate::commands::cache) struct WebSharedCacheStats {
    pub(in crate::commands::cache) pinned_package_bytes: u64,
    pub(in crate::commands::cache) unpinned_package_bytes: u64,
    pub(in crate::commands::cache) pinned_package_roots: usize,
    pub(in crate::commands::cache) unpinned_package_roots: usize,
    pub(in crate::commands::cache) project_refs: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(in crate::commands::cache) struct WebProjectCacheStats {
    pub(in crate::commands::cache) cas_bytes: u64,
    pub(in crate::commands::cache) tarball_bytes: u64,
    pub(in crate::commands::cache) resolution_bytes: u64,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(in crate::commands::cache) struct WebProjectPruneStats {
    pub(in crate::commands::cache) cas_files: usize,
    pub(in crate::commands::cache) tarball_files: usize,
    pub(in crate::commands::cache) resolution_files: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(in crate::commands::cache) struct GenericPruneStats {
    pub(in crate::commands::cache) files: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::commands::cache) enum CacheAction {
    Status,
    Clean,
    Prune,
}

impl CacheAction {
    pub(in crate::commands::cache) fn parse(raw: &str) -> Result<Self> {
        match raw {
            "status" => Ok(Self::Status),
            "clean" => Ok(Self::Clean),
            "prune" => Ok(Self::Prune),
            other => bail!("unknown cache action: {other}"),
        }
    }

    pub(in crate::commands::cache) fn includes_build_target(self, target: CacheTarget) -> bool {
        match self {
            Self::Status => target.includes(CacheTarget::Build),
            Self::Clean | Self::Prune => target == CacheTarget::Build,
        }
    }
}
