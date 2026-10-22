//! App framework qualification rows.
//! Các dòng qualification framework App.

use super::super::super::{
    EV_E2E_APP_FLUTTER, EV_E2E_APP_SWIFT, EV_RN_GATE, EV_WIZARD, FrameworkRecord, FrameworkStatus,
};

pub(in crate::commands::framework_records::implementation::records) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "app",
        framework: "flutter",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_APP_FLUTTER,
    },
    FrameworkRecord {
        core: "app",
        framework: "kotlin",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "dep_gate app/kotlin install+list cells + canary (add/remove/update: no runner)",
    },
    FrameworkRecord {
        core: "app",
        framework: "swift",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_APP_SWIFT,
    },
    FrameworkRecord {
        core: "app",
        framework: "react-native",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_RN_GATE,
    },
    FrameworkRecord {
        core: "app",
        framework: "maui",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "app",
        framework: "tauri",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "app",
        framework: "multi",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (per-platform lanes gate individually)",
    },
];
