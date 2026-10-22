//! Upgrades v2 metadata and package provenance to schema v3.
//! Nâng metadata và provenance package v2 lên schema v3.

use crate::Lockfile;
use crate::schema::{LOCKFILE_SCHEMA_VERSION, Provenance, SOURCE_KIND_REGISTRY_IMPORT};

/// Migrate lockfile v2 to v3 — Migrate lockfile v2 sang v3
///
/// v2 files carry no ecosystem data, so after migration every package has
/// `ecosystem = other` (the serde default applied during the v2 parse) and
/// `provenance = registry-import` where provenance was empty. Ecosystem is
/// NOT force-reset: running the migration on a v3 lock must not clobber
/// existing ecosystem data.
/// File v2 không mang dữ liệu ecosystem, nên sau migration mọi package có
/// `ecosystem = other` (mặc định serde khi parse v2) và
/// `provenance = registry-import` ở chỗ provenance còn trống. KHÔNG ép
/// reset ecosystem: chạy migration trên lock v3 không được xoá dữ liệu
/// ecosystem sẵn có.
pub fn migrate_v2_to_v3(mut lockfile: Lockfile) -> Lockfile {
    lockfile.version = LOCKFILE_SCHEMA_VERSION.to_string();

    for pkg in &mut lockfile.packages {
        if pkg.provenance.is_none() {
            pkg.provenance = Some(Provenance {
                source_kind: SOURCE_KIND_REGISTRY_IMPORT.to_string(),
                tool: None,
                imported_from: None,
            });
        }
    }

    lockfile
}
