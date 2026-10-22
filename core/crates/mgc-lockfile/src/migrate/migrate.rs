//! Coordinates automatic v1/v2/v3 upgrades; v4 stays explicit.
//! Điều phối nâng cấp tự động v1/v2/v3; v4 luôn cần chọn tường minh.

use super::legacy_v1::parse_lockfile_v1;
use super::v1_to_v2::migrate_v1_to_v2;
use super::v2_to_v3::migrate_v2_to_v3;
use super::version_detection::detect_lockfile_version;
use crate::{Lockfile, LockfileError, LockfileResult};

/// Auto-upgrade lockfile to the latest v3 schema (v1/v2 → v3) — Tự động nâng cấp lockfile lên schema v3 mới nhất (v1/v2 → v3).
///
/// v4 is NEVER an automatic target: the major bump changes identity,
/// digest coverage, and signature shape, so migration stays an explicit
/// `mgc migrate lock --to v4` decision. A v4 document here is an error
/// pointing at the v4 reader.
/// (v4 KHÔNG BAO GIỜ là đích tự động: bump major đổi identity, phủ
/// digest và shape chữ ký, nên migration là quyết định tường minh.)
pub fn auto_upgrade_lockfile(toml_str: &str) -> LockfileResult<Lockfile> {
    let version = detect_lockfile_version(toml_str)?;

    match version {
        1 => {
            let lockfile_v1 = parse_lockfile_v1(toml_str)?;
            // Chain both steps so old locks land on the canonical v3 graph.
            // Nối cả hai bước để lock cũ về đồ thị canonical v3.
            Ok(migrate_v2_to_v3(migrate_v1_to_v2(lockfile_v1)?))
        }
        2 => {
            let lockfile = crate::parser::parse_lockfile(toml_str)?;
            Ok(migrate_v2_to_v3(lockfile))
        }
        3 => crate::parser::parse_lockfile(toml_str),
        4 => Err(LockfileError::ParseError(
            "v4 lockfile: use the v4 reader (migration is explicit, never automatic)".to_string(),
        )),
        _ => Err(LockfileError::ParseError(format!(
            "unsupported version: {}",
            version
        ))),
    }
}
