//! Hardware framework qualification rows.
//! Các dòng qualification framework Hardware.

use super::super::super::{EV_E2E_HARDWARE, FrameworkRecord, FrameworkStatus};

pub(in crate::commands::framework_records::implementation::records) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "hardware",
        framework: "bench",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_HARDWARE,
    },
    FrameworkRecord {
        core: "hardware",
        framework: "optimizer",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_HARDWARE,
    },
];
