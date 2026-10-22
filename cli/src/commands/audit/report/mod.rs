//! Shared audit report rendering facade.
//! Facade render audit report dùng chung.

use super::OutputFormat;

#[path = "report.rs"]
mod report;

#[cfg(test)]
pub(crate) use report::finding_table_labels;
pub(crate) use report::print_audit_report;
