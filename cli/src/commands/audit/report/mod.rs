//! Shared audit report rendering facade.
//! Facade render audit report dùng chung.

use super::OutputFormat;

#[path = "report.rs"]
mod report;

pub(crate) use report::print_audit_report;
#[cfg(test)]
pub(crate) use report::{advisory_id_label, finding_table_labels, is_cve_id};
