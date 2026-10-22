//! Lockfile migration diagnostics.
//! Lỗi migrate lockfile.

use anyhow::Error;
use anyhow::anyhow;

/// `mgc migrate` does not know this target (only `--to v4` exists).
pub fn migrate_unknown_target(to: &str) -> Error {
    anyhow!("unknown migrate target '{to}' (only `--to v4` exists)")
}

/// `mgc migrate lock` found no mgc.lock to migrate.
pub fn migrate_no_lockfile(root: &std::path::Path) -> Error {
    anyhow!("no mgc.lock in '{}' — nothing to migrate", root.display())
}

/// `mgc migrate lock` refused: the v1/v2/v3 → v4 migration would drop
/// data (edges, peers, sources, or installer SRI). The existing mgc.lock
/// was left unchanged — re-resolve the lossy lanes first, then migrate.
/// (Migrate từ chối vì hao hụt dữ liệu — giữ nguyên lock cũ.)
pub fn migrate_v4_lossy(warnings: &[String]) -> Error {
    anyhow!(
        "schema v4 migration refused (lossy — existing mgc.lock left unchanged): {}",
        warnings.join("; ")
    )
}

/// `mgc migrate lock` emitted v4 text that does not round-trip into the
/// identical document — a serializer drift that must never hit disk.
/// (Text v4 ghi ra không round-trip y hệt — không được ghi đĩa.)
pub fn migrate_v4_roundtrip_mismatch() -> Error {
    anyhow!(
        "migrated v4 document does not round-trip identically — refusing to write (serializer drift)"
    )
}

/// `mgc migrate lock` cannot start from this schema version.
pub fn migrate_unsupported_version(version: u8) -> Error {
    anyhow!("cannot migrate lockfile schema version {version} (supported sources: v1, v2, v3)")
}
