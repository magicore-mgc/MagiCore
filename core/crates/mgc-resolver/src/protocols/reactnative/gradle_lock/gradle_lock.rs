//! Gradle lock data type and stable parser/collapse entry points.
//! Kiểu dữ liệu Gradle lock và entry point parser/gộp ổn định.

use mgc_types::MgResult;

/// One pinned Gradle coordinate from `gradle.lockfile`.
/// Một tọa độ Gradle đã ghim trong `gradle.lockfile`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradleLockEntry {
    pub group: String,
    pub artifact: String,
    pub version: String,
    pub configuration: String,
}

impl GradleLockEntry {
    /// Maven coordinate name (`group:artifact`).
    /// Tên tọa độ Maven (`group:artifact`).
    pub fn coordinate(&self) -> String {
        format!("{}:{}", self.group, self.artifact)
    }
}

/// Parse `gradle.lockfile`, failing closed on a malformed pin.
/// Parse `gradle.lockfile`, fail-closed nếu pin sai định dạng.
pub fn parse_gradle_lockfile(text: &str) -> MgResult<Vec<GradleLockEntry>> {
    super::parse::parse_entries(text).map_err(Into::into)
}

/// Collapse duplicate configurations to one exact version per coordinate.
/// Gộp configuration trùng thành một version chính xác cho mỗi tọa độ.
pub fn collapse_gradle_pins(entries: &[GradleLockEntry]) -> MgResult<Vec<(String, String)>> {
    super::collapse::collapse_entries(entries).map_err(Into::into)
}

#[cfg(test)]
#[path = "test/gradle_lock.rs"]
mod tests;
