//! Typed outcome of native, cached, fetched or optional-missing resolution.
//! Kết quả có kiểu cho resolve native, cache, registry hoặc thiếu optional.

use std::path::PathBuf;

/// Kết quả resolve scaffold - typed thay vì bool
#[derive(Debug, Clone)]
pub enum ScaffoldResolveStatus {
    /// Built by an MGC native processor; this is not a bundled framework archive.
    /// Được dựng bởi processor native MGC, không phải archive framework nhúng.
    Embedded { layer: String },
    /// Cache hit (đã fetch trước)
    CacheHit {
        layer: String,
        version: Option<String>,
        path: PathBuf,
    },
    /// Vừa fetch từ registry
    Fetched {
        layer: String,
        version: String,
        path: PathBuf,
    },
    /// Missing nhưng optional (không block)
    OptionalMissing { layer: String, reason: String },
}

impl ScaffoldResolveStatus {
    /// Check if resolved successfully (native processor/cache/fetched).
    /// Kiểm tra layer đã được xử lý bằng processor, cache hoặc registry.
    pub fn is_available(&self) -> bool {
        matches!(
            self,
            ScaffoldResolveStatus::Embedded { .. }
                | ScaffoldResolveStatus::CacheHit { .. }
                | ScaffoldResolveStatus::Fetched { .. }
        )
    }

    /// Get layer name
    pub fn layer(&self) -> &str {
        match self {
            ScaffoldResolveStatus::Embedded { layer }
            | ScaffoldResolveStatus::CacheHit { layer, .. }
            | ScaffoldResolveStatus::Fetched { layer, .. }
            | ScaffoldResolveStatus::OptionalMissing { layer, .. } => layer,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
