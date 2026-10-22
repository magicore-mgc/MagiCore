//! Converts only one core lane's resolved package rows into v4.
//! Chỉ chuyển các row package đã resolve của một lane core sang v4.

use super::super::lossy_migration::LossyMigration;
use super::super::v3_to_v4::migrate_v3_to_v4;
use crate::schema::LOCKFILE_SCHEMA_VERSION;
use crate::{Lockfile, Package};

/// Convert one lane's freshly resolved v3 rows into v4 packages + sources,
/// refusing lossy rows fail-closed (shared by every adapter write path).
/// The lane rows — not a merged multi-core lock — are converted, so a
/// lane can never silently inherit another lane's lossy history.
/// (Chuyển row v3 vừa resolve của một lane thành package + source v4,
/// từ chối row hao hụt fail-closed (dùng chung mọi đường ghi adapter).
/// Chỉ convert row của lane, không bao giờ lặng lẽ thừa hưởng hao hụt
/// của lane khác.)
pub fn lane_rows_to_v4(
    packages: Vec<Package>,
    generator: &str,
) -> Result<(Vec<crate::canonical::PackageV4>, Vec<crate::v4::SourceRef>), LossyMigration> {
    let lane_lock = Lockfile {
        version: LOCKFILE_SCHEMA_VERSION.to_string(),
        metadata: crate::schema::LockfileMetadata {
            generated_at: String::new(),
            generator: generator.to_string(),
            lockfile_hash: String::new(),
            signer: None,
            dependency_ownership: Vec::new(),
        },
        root_dependencies: Vec::new(),
        root_dependencies_by_owner: Default::default(),
        packages,
        workspace: None,
        optimizer_profile: None,
    };
    let (v4, warnings) = migrate_v3_to_v4(lane_lock).map_err(|error| LossyMigration {
        warnings: vec![error.to_string()],
    })?;
    if !warnings.is_empty() {
        return Err(LossyMigration { warnings });
    }
    Ok((v4.packages, v4.sources))
}
