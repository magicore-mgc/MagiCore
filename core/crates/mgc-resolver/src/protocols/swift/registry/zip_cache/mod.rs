//! Bounded archive cache implementation and configuration.
//! Triển khai cache archive có giới hạn và cấu hình.

mod cache_limits;
mod error;
mod zip_cache;

pub(super) use zip_cache::SwiftZipCache;
