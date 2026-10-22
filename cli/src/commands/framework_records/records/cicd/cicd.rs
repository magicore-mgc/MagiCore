//! CI/CD framework qualification rows.
//! Các dòng qualification framework CI/CD.

use super::super::super::{EV_WIZARD, FrameworkRecord, FrameworkStatus};

pub(in crate::commands::framework_records::implementation::records) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "cicd",
        framework: "argocd",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "cicd",
        framework: "aws",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "cicd",
        framework: "cloudflare",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "cicd",
        framework: "gcp",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "cicd",
        framework: "github-actions",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
];
