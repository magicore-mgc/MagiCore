//! Generic project dependency-deduplication settings.
//! Cấu hình dedupe dependency dùng chung cấp project.

#[path = "dedupe.rs"]
mod dedupe;

pub use dedupe::DedupeConfig;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
