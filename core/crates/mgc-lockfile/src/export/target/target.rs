//! Export destination set.
//! Tập đích export.

/// Export destinations supported in Phase 1 — Đích export hỗ trợ ở Phase 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportTarget {
    /// Cargo.lock (TOML) for the cargo toolchain — Cargo.lock (TOML) cho toolchain cargo.
    CargoLock,
    /// requirements.txt (pip line format) — requirements.txt (định dạng dòng pip).
    RequirementsTxt,
    /// go.sum placeholder (comment skeleton) — go.sum placeholder (comment skeleton).
    GoSum,
    /// package-lock.json (npm lockfileVersion 3) — package-lock.json (npm lockfileVersion 3).
    PackageLock,
}
