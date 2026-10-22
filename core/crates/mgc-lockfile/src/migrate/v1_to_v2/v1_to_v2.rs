//! Converts v1 records while preserving absent package integrity.
//! Chuyển record v1 và giữ nguyên việc thiếu integrity package.

use super::super::legacy_v1::LockfileV1;
use crate::{Lockfile, LockfileResult, Package};

/// Migrate lockfile v1 to v2 — Migrate lockfile v1 sang v2
///
/// Emits the v2 shape on purpose (version pinned to "2") so the step stays
/// composable; `auto_upgrade_lockfile` chains it into `migrate_v2_to_v3`.
/// Cố ý xuất shape v2 (version ghim "2") để bước này ghép được;
/// `auto_upgrade_lockfile` sẽ nối tiếp bằng `migrate_v2_to_v3`.
pub fn migrate_v1_to_v2(lockfile_v1: LockfileV1) -> LockfileResult<Lockfile> {
    let mut lockfile_v2 = Lockfile::new();
    // Pin the intermediate schema version — the name of this function is a
    // contract (v1 → v2), the final bump happens in migrate_v2_to_v3.
    // Ghim version schema trung gian — tên hàm là hợp đồng (v1 → v2),
    // bước nâng cuối nằm ở migrate_v2_to_v3.
    lockfile_v2.version = "2".to_string();

    // Migrate packages — Chuyển các package.
    for pkg_v1 in lockfile_v1.packages {
        let pkg_v2 = Package {
            name: pkg_v1.name.clone(),
            version: pkg_v1.version.clone(),
            resolved: pkg_v1.resolved.clone(),
            // V1 has no package-byte digest; preserve absence rather than invent evidence.
            // Lock v1 không có digest byte package; giữ trạng thái thiếu thay vì bịa bằng chứng.
            integrity: String::new(),
            dependencies: pkg_v1.dependencies.clone(),
            // v3 fields keep their defaults (ecosystem=other, provenance=None);
            // migrate_v2_to_v3 fills provenance right after.
            // Field v3 giữ mặc định (ecosystem=other, provenance=None);
            // migrate_v2_to_v3 sẽ điền provenance ngay sau đó.
            ..Default::default()
        };

        lockfile_v2.add_package(pkg_v2);
    }

    Ok(lockfile_v2)
}
