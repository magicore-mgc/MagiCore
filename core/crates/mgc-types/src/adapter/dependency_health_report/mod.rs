//! Dependency Health Report owner exports — xuất báo cáo độ mới dependency.

mod dependency_health_report;
pub use dependency_health_report::DependencyHealthReport;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
