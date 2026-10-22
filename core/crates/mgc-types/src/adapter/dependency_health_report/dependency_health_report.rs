//! Aggregate package freshness counts and outdated entries. — Thống kê freshness package và danh sách package outdated.

use super::super::OutdatedDependency;

/// Dependency freshness report — version drift across audited packages.
/// Báo cáo độ tươi dependency — lệch version trong các package đã kiểm tra.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DependencyHealthReport {
    /// Total packages checked (up to date + outdated).
    /// Tổng package đã kiểm tra (cả mới lẫn cũ).
    pub packages_checked: usize,
    pub outdated_count: usize,
    pub outdated: Vec<OutdatedDependency>,
}
