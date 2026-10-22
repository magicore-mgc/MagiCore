//! Cloud framework qualification rows.
//! Các dòng qualification framework Cloud.

use super::super::super::{EV_E2E_CLO_TERRAFORM, EV_WIZARD, FrameworkRecord, FrameworkStatus};

pub(in crate::commands::framework_records::implementation::records) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "clo",
        framework: "terraform",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_CLO_TERRAFORM,
    },
    FrameworkRecord {
        core: "clo",
        framework: "cdk",
        status: FrameworkStatus::NativeEngine,
        evidence: "clo lane branches pre-gate into the embedded web engine",
    },
    FrameworkRecord {
        core: "clo",
        framework: "pulumi",
        status: FrameworkStatus::NativeEngine,
        evidence: "clo lane branches pre-gate into the embedded web engine",
    },
    FrameworkRecord {
        core: "clo",
        framework: "cloudflare",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
];
