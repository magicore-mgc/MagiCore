//! MagiCore shared package cache API.
//! API cache package dùng chung của MagiCore.

pub mod memory;
mod package_cache;

pub use package_cache::{CacheEntry, PackageCache};
