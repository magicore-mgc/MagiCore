//! Wire GitHub Actions finding-to-report projection.
//! Nối phép chiếu finding GitHub Actions sang audit report.

mod report;

pub(super) use report::build_report;
