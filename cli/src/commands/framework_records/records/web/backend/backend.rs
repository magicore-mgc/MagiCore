//! Web backend framework qualification rows.
//! Các dòng qualification framework Web backend.

use super::super::super::super::{EV_E2E_WEB_BACKEND, FrameworkRecord, FrameworkStatus};

pub(in crate::commands::framework_records::implementation::records::web) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "web",
        framework: "actix-web",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "axum",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "django",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "dotnet",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "dotnet-minimal",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "dotnet-webapi",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "echo",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "fastapi",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "fiber",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "flask",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "gin",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "go",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "java",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "python",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "quarkus",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "rust",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "spring-boot",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
];
