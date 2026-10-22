//! Install accounting result and explicit cache-ownership classification. — Kết quả thống kê install và phân loại rõ owner của cache.

use crate::package::PackageId;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct InstallSummary {
    pub added: Vec<PackageId>,
    pub bytes_from_cache: u64,
    pub duration_ms: u64,
    /// P0-6 (2026-09-11): cache accounting mode — HONEST labeling so a
    /// zero byte-count is never misread as "no cache". `mgc-store` =
    /// measured through MagiCore's content-addressable store;
    /// `delegated` = the native toolchain owns its cache (cargo/uv/go
    /// module cache) and mgc does not count its bytes.
    /// P0-6: chế độ tính cache — ghi nhãn TRUNG THỰC để byte-count = 0
    /// không bao giờ bị đọc nhầm "không có cache". `mgc-store` = đo qua
    /// content-addressable store của MagiCore; `delegated` = toolchain
    /// gốc giữ cache của nó (cargo/uv/go module cache) và mgc không
    /// đếm byte của nó.
    #[serde(default)]
    pub cache_mode: InstallCacheMode,
}

/// How the install pipeline accounted for cache bytes.
/// Cách pipeline install tính byte cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum InstallCacheMode {
    /// Measured via the mgc content-addressable store.
    /// Đo qua content-addressable store của mgc.
    #[default]
    MgCStore,
    /// Native toolchain cache owns the bytes (delegation, not mgc).
    /// Cache toolchain gốc giữ byte (ủy quyền, không phải mgc).
    Delegated,
}
