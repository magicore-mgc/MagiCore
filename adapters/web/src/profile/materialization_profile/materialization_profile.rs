//! Materialized package, file, directory, and link counters.
//! Bộ đếm package, file, thư mục và kiểu liên kết đã materialize.

use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub struct MaterializationProfile {
    pub enabled: bool,
    pub packages_linked: AtomicUsize,
    pub files_seen: AtomicUsize,
    pub directories_seen: AtomicUsize,
    pub hardlinks: AtomicUsize,
    pub copies: AtomicUsize,
    pub reflinks: AtomicUsize,
    pub symlinks: AtomicUsize,
}

impl MaterializationProfile {
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

    pub fn record_package_linked(&self) {
        if self.enabled {
            self.packages_linked.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_directory(&self) {
        if self.enabled {
            self.directories_seen.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_file(&self) {
        if self.enabled {
            self.files_seen.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_hardlink(&self) {
        if self.enabled {
            self.hardlinks.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_copy(&self) {
        if self.enabled {
            self.copies.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_symlink(&self) {
        if self.enabled {
            self.symlinks.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_reflink(&self) {
        if self.enabled {
            self.reflinks.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn flush(&self) {
        if !self.enabled {
            return;
        }
        eprintln!(
            concat!(
                "[magicore:web:materialize-profile] ",
                "packages_linked={} dirs={} files={} hardlinks={} copies={} ",
                "reflinks={} symlinks={}"
            ),
            self.packages_linked.load(Ordering::Relaxed),
            self.directories_seen.load(Ordering::Relaxed),
            self.files_seen.load(Ordering::Relaxed),
            self.hardlinks.load(Ordering::Relaxed),
            self.copies.load(Ordering::Relaxed),
            self.reflinks.load(Ordering::Relaxed),
            self.symlinks.load(Ordering::Relaxed),
        );
    }
}

#[cfg(test)]
#[path = "test/materialization_profile.rs"]
mod tests;
