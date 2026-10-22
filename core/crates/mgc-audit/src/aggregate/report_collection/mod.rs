//! Report-fold owner wiring — nối owner gom số liệu và findings.

mod report_collection;

pub(in crate::aggregate) use report_collection::{ReportSummary, collect_reports};
