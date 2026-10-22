//! Own prune configuration and scheduling stamps.
//! Quản lý cấu hình prune và stamp lịch chạy.

use std::path::{Path, PathBuf};

use crate::cache::current_unix_secs;
use crate::manifest::atomic_write;
use mgc_types::MgResult;

pub fn shared_cache_prune_interval_secs() -> u64 {
    std::env::var("MAGICORE_WEB_CACHE_PRUNE_INTERVAL_SECS")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|ttl| *ttl > 0)
        .unwrap_or(6 * 60 * 60)
}

pub fn shared_cache_max_age_secs() -> u64 {
    std::env::var("MAGICORE_WEB_CACHE_MAX_AGE_SECS")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|ttl| *ttl > 0)
        .unwrap_or(7 * 24 * 60 * 60)
}

pub fn shared_cache_max_bytes() -> u64 {
    std::env::var("MAGICORE_WEB_CACHE_MAX_BYTES")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .unwrap_or(2 * 1024 * 1024 * 1024)
}

pub fn shared_cache_prune_stamp_path(root: &Path) -> PathBuf {
    root.join(".gc-stamp")
}

pub fn project_cache_prune_stamp_path(root: &Path) -> PathBuf {
    root.join(".project-gc-stamp")
}

pub fn shared_cache_prune_due(root: &Path) -> bool {
    prune_stamp_due(&shared_cache_prune_stamp_path(root))
}

pub fn project_cache_prune_due(root: &Path) -> bool {
    prune_stamp_due(&project_cache_prune_stamp_path(root))
}

fn prune_stamp_due(stamp: &Path) -> bool {
    let Ok(metadata) = std::fs::metadata(stamp) else {
        return true;
    };
    let Ok(modified) = metadata.modified() else {
        return true;
    };
    let Ok(elapsed) = modified.elapsed() else {
        return true;
    };
    elapsed >= std::time::Duration::from_secs(shared_cache_prune_interval_secs())
}

pub fn write_shared_cache_prune_stamp(root: &Path) -> MgResult<()> {
    write_prune_stamp(&shared_cache_prune_stamp_path(root))
}

pub fn write_project_cache_prune_stamp(root: &Path) -> MgResult<()> {
    write_prune_stamp(&project_cache_prune_stamp_path(root))
}

fn write_prune_stamp(stamp: &Path) -> MgResult<()> {
    let data = current_unix_secs().to_string();
    atomic_write(stamp, data.as_bytes())?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
