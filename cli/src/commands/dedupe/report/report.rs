//! Dedupe report renderer — preserves the current JSON schema and terminal text.
//! Renderer report dedupe — giữ schema JSON và nội dung terminal hiện tại.
use crate::commands::dedupe::{DedupeArgs, DedupeReport};

use super::error::DedupeReportError;

/// Render the dedupe report (shared by dry-run and real runs).
/// (In báo cáo dedupe — chung cho dry-run và chạy thật.)
pub(in crate::commands::dedupe) fn print_report(
    args: &DedupeArgs,
    report: &DedupeReport,
) -> std::result::Result<(), DedupeReportError> {
    if args.json {
        println!("{}", report_json(report)?);
    } else {
        println!("Dedupe report:");
        println!("  Before: {} instances", report.before_instances);
        println!("  After:  {} instances", report.after_instances);
        println!("  Merged: {} duplicates", report.merged);
        if report.disk_saved_bytes > 0 {
            println!(
                "  Vstore logical payload bytes in removed paths: {}",
                report.disk_saved_bytes
            );
        }
        if args.dry_run {
            println!("(dry-run — no changes applied)");
        }
    }

    Ok(())
}

fn report_json(report: &DedupeReport) -> std::result::Result<String, DedupeReportError> {
    serde_json::to_string_pretty(report).map_err(DedupeReportError::JsonSerialization)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
