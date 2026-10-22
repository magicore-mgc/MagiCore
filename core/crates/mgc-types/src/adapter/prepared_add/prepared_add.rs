//! Prepared package identity and selected version range returned by native add planning. — Identity package và version range đã chuẩn bị mà native add planning trả về.

use crate::package::{PackageId, VersionRange};
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PreparedAdd {
    pub id: PackageId,
    pub range: VersionRange,
}
