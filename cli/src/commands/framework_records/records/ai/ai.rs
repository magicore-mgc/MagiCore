//! AI framework qualification rows.
//! Các dòng qualification framework AI.

use super::super::super::{EV_DEPGATE, EV_WIZARD, FrameworkRecord, FrameworkStatus};

pub(in crate::commands::framework_records::implementation::records) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "ai",
        framework: "python-agent",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_DEPGATE,
    },
    FrameworkRecord {
        core: "ai",
        framework: "mcp-server",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
];
