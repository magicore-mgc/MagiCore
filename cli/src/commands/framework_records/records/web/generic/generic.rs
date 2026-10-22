//! Web generic framework qualification rows.
//! Các dòng qualification framework Web generic.

use super::super::super::super::{
    EV_SCAFFOLD_WEB, EV_WEB_ENGINE, EV_WIZARD, FrameworkRecord, FrameworkStatus,
};

pub(in crate::commands::framework_records::implementation::records::web) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "web",
        framework: "vanilla",
        status: FrameworkStatus::NativeEngine,
        evidence: EV_WEB_ENGINE,
    },
    FrameworkRecord {
        core: "web",
        framework: "ts",
        status: FrameworkStatus::NativeEngine,
        evidence: EV_WEB_ENGINE,
    },
    FrameworkRecord {
        core: "web",
        framework: "node",
        status: FrameworkStatus::NativeEngine,
        evidence: EV_WEB_ENGINE,
    },
    FrameworkRecord {
        core: "web",
        framework: "backend",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "custom",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "web",
        framework: "frontend",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "fullstack",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "laravel",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_SCAFFOLD_WEB,
    },
    FrameworkRecord {
        core: "web",
        framework: "monorepo",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "php",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard language branch (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "symfony",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_SCAFFOLD_WEB,
    },
    FrameworkRecord {
        core: "web",
        framework: "vue-laravel",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
];
