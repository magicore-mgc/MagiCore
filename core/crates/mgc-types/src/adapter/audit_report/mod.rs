//! Audit Report owner exports — xuất kiểu báo cáo kiểm toán.

mod audit_report;
pub use audit_report::AuditReport;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
