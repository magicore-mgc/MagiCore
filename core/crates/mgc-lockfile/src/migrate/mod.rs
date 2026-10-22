//! Preserves the public migration API while wiring focused migration owners.
//! Giữ API migration công khai và nối tới các owner chuyên trách.

mod finalize_migrated_v4;
mod lane_rows_to_v4;
mod legacy_v1;
mod lossy_migration;
mod migrate;
mod read_v4_merge_base;
mod v1_to_v2;
mod v2_to_v3;
mod v3_to_v4;
mod verify_v4_merge_base;
mod version_detection;

pub use self::finalize_migrated_v4::finalize_migrated_v4;
pub use self::lane_rows_to_v4::lane_rows_to_v4;
pub use self::legacy_v1::{LockfileV1, PackageV1, parse_lockfile_v1};
pub use self::lossy_migration::LossyMigration;
pub use self::migrate::auto_upgrade_lockfile;
pub use self::read_v4_merge_base::read_v4_merge_base;
pub use self::v1_to_v2::migrate_v1_to_v2;
pub use self::v2_to_v3::migrate_v2_to_v3;
pub use self::v3_to_v4::migrate_v3_to_v4;
pub use self::verify_v4_merge_base::verify_v4_merge_base;
pub use self::version_detection::detect_lockfile_version;
