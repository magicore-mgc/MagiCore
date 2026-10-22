//! Model parsed advisory success separately from scanner-failed reports.
//! Tách kết quả advisory parse thành công khỏi report scanner bị lỗi.

use mgc_types::Vulnerability;
use mgc_types::adapter::AuditReport;

#[derive(Debug)]
pub(in crate::audit) enum AdvisoryQueryOutcome {
    ScannerFailed(AuditReport),
    Parsed(Vec<Vulnerability>),
}
