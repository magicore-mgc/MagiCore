//! Concurrent download and extraction counters with bounded slow-item summaries.
//! Bộ đếm download/extract đồng thời và danh sách item chậm có giới hạn.

use mgc_types::PackageId;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

#[derive(Default)]
pub struct PipelineProfile {
    pub enabled: bool,
    pub package_count: AtomicU64,
    pub tarball_bytes: AtomicU64,
    pub download_ms_total: AtomicU64,
    pub extract_ms_total: AtomicU64,
    pub download_ms_max: AtomicU64,
    pub extract_ms_max: AtomicU64,
    pub slowest_downloads: Mutex<Vec<(u64, String, u64)>>,
    pub slowest_extracts: Mutex<Vec<(u64, String)>>,
}

impl PipelineProfile {
    pub fn from_env() -> Self {
        let enabled = std::env::var("MAGICORE_WEB_PROFILE_INSTALL")
            .ok()
            .map(|value| value.trim().to_ascii_lowercase())
            .map(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false);
        Self {
            enabled,
            ..Default::default()
        }
    }

    fn downloads_guard(&self) -> MutexGuard<'_, Vec<(u64, String, u64)>> {
        self.slowest_downloads
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn extracts_guard(&self) -> MutexGuard<'_, Vec<(u64, String)>> {
        self.slowest_extracts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn record_download(
        &self,
        package: &PackageId,
        bytes: u64,
        elapsed_ms: u64,
        queue_wait_ms: u64,
        io_ms: u64,
    ) {
        if !self.enabled {
            return;
        }
        self.package_count.fetch_add(1, Ordering::Relaxed);
        self.tarball_bytes.fetch_add(bytes, Ordering::Relaxed);
        self.download_ms_total
            .fetch_add(elapsed_ms, Ordering::Relaxed);
        self.download_ms_max
            .fetch_max(elapsed_ms, Ordering::Relaxed);
        self.record_slowest_download(package, bytes, elapsed_ms, queue_wait_ms, io_ms);
    }

    pub fn record_extract(&self, package: &PackageId, elapsed_ms: u64) {
        if !self.enabled {
            return;
        }
        self.extract_ms_total
            .fetch_add(elapsed_ms, Ordering::Relaxed);
        self.extract_ms_max.fetch_max(elapsed_ms, Ordering::Relaxed);
        self.record_slowest_extract(package, elapsed_ms);
    }

    pub fn record_slowest_download(
        &self,
        package: &PackageId,
        bytes: u64,
        elapsed_ms: u64,
        queue_wait_ms: u64,
        io_ms: u64,
    ) {
        let mut guard = self.downloads_guard();
        guard.push((
            elapsed_ms,
            format!("{} queue_wait={}ms io={}ms", package, queue_wait_ms, io_ms),
            bytes,
        ));
        guard.sort_by_key(|entry| std::cmp::Reverse(entry.0));
        guard.truncate(5);
    }

    pub fn record_slowest_extract(&self, package: &PackageId, elapsed_ms: u64) {
        let mut guard = self.extracts_guard();
        guard.push((elapsed_ms, package.to_string()));
        guard.sort_by_key(|entry| std::cmp::Reverse(entry.0));
        guard.truncate(5);
    }

    pub fn flush(&self) {
        if !self.enabled {
            return;
        }
        eprintln!(
            concat!(
                "[magicore:web:pipeline-profile] ",
                "packages={} bytes={} download_ms_total={} download_ms_max={} ",
                "extract_ms_total={} extract_ms_max={}"
            ),
            self.package_count.load(Ordering::Relaxed),
            self.tarball_bytes.load(Ordering::Relaxed),
            self.download_ms_total.load(Ordering::Relaxed),
            self.download_ms_max.load(Ordering::Relaxed),
            self.extract_ms_total.load(Ordering::Relaxed),
            self.extract_ms_max.load(Ordering::Relaxed),
        );
        for (elapsed_ms, package, bytes) in self.downloads_guard().iter() {
            eprintln!(
                "[magicore:web:pipeline-profile] slow_download package={} elapsed={}ms bytes={}",
                package, elapsed_ms, bytes
            );
        }
        for (elapsed_ms, package) in self.extracts_guard().iter() {
            eprintln!(
                "[magicore:web:pipeline-profile] slow_extract package={} elapsed={}ms",
                package, elapsed_ms
            );
        }
    }
}

#[cfg(test)]
#[path = "test/pipeline_profile.rs"]
mod tests;
