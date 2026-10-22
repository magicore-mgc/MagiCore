//! A package freshness result kept separate from security findings. — Kết quả package outdated được tách khỏi security finding.

use crate::package::PackageId;
use crate::version::Version;
/// One outdated dependency entry — version drift, NOT a CVE.
/// Một entry dependency cũ — lệch version, KHÔNG phải CVE.
/// `flutter pub outdated` reports dependency freshness; mixing it into
/// security findings would misrepresent audit results.
/// `flutter pub outdated` báo độ tươi dependency; trộn vào finding bảo mật
/// sẽ xuyên tạc kết quả audit.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OutdatedDependency {
    pub package: PackageId,
    /// Latest version available on the registry.
    /// Version mới nhất trên registry.
    pub latest_version: Version,
}
