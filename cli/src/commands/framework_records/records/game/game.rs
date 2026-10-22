//! Game framework qualification rows.
//! Các dòng qualification framework Game.

use super::super::super::{EV_DEPGATE, FrameworkRecord, FrameworkStatus};

pub(in crate::commands::framework_records::implementation::records) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "game",
        framework: "bevy",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_DEPGATE,
    },
    FrameworkRecord {
        core: "game",
        framework: "godot",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "no package manager (lane fail-closed); canary godot_*",
    },
    FrameworkRecord {
        core: "game",
        framework: "unity",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "UPM CLI is P2 (lane fail-closed)",
    },
    FrameworkRecord {
        core: "game",
        framework: "unreal",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "no package manager (lane fail-closed)",
    },
];
