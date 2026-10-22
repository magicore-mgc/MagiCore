//! Wires focused v3-to-v4 conversion owners and their tests.
//! Nối các owner chuyển đổi v3 sang v4 cùng test tương ứng.

mod artifact_integrity;
mod package_conversion;
mod registry_source_catalog;
mod types;
mod v3_to_v4;
use self::artifact_integrity::carry_install_sri;
use self::package_conversion::convert_packages;
use self::registry_source_catalog::{build_source_catalog, package_source_id};
use self::types::SourceCatalog;
pub use self::v3_to_v4::migrate_v3_to_v4;
#[cfg(test)]
mod test;
