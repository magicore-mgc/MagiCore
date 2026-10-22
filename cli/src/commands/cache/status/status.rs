//! Render cache status and byte sizes.
//! Hiển thị status cache và định dạng dung lượng.

use crate::commands::cache::{
    CacheEntry,
    filesystem::path_size,
    prune::{web_project_cache_stats, web_shared_cache_stats},
    storage_budget::print_storage_budget,
};
use anyhow::Result;

pub(in crate::commands::cache) fn print_status(entries: &[CacheEntry]) -> Result<()> {
    if entries.is_empty() {
        println!("No cache paths found.");
    } else {
        for entry in entries {
            let bytes = path_size(&entry.path);
            let exists = entry.path.exists();
            println!(
                "{}\t{}\t{}\t{}",
                entry.label,
                human_bytes(bytes),
                if exists { "exists" } else { "missing" },
                entry.path.display()
            );
            if entry.label == "shared" && entry.path.ends_with("cache/web") {
                let stats = web_shared_cache_stats(&entry.path);
                println!(
                    "shared:web:pinned\t{}\t{} roots\t{} refs",
                    human_bytes(stats.pinned_package_bytes),
                    stats.pinned_package_roots,
                    stats.project_refs
                );
                println!(
                    "shared:web:unpinned\t{}\t{} roots",
                    human_bytes(stats.unpinned_package_bytes),
                    stats.unpinned_package_roots
                );
            } else if entry.label == "project" && entry.path.ends_with(".magicore/cache/web") {
                let stats = web_project_cache_stats(&entry.path);
                println!("project:web:cas\t{}\tcas", human_bytes(stats.cas_bytes));
                println!(
                    "project:web:tarballs\t{}\tcache",
                    human_bytes(stats.tarball_bytes)
                );
                println!(
                    "project:web:resolutions\t{}\tresolutions",
                    human_bytes(stats.resolution_bytes)
                );
            }
        }
    }

    print_storage_budget()?;
    Ok(())
}

pub(in crate::commands::cache) fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit = 0usize;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}
