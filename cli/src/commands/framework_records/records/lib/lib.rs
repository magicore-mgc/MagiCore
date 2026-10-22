//! Lib framework qualification rows.
//! Các dòng qualification framework Lib.

use super::super::super::{EV_LIB_PIPELINE, FrameworkRecord, FrameworkStatus};

pub(in crate::commands::framework_records::implementation::records) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "lib",
        framework: "ts",
        status: FrameworkStatus::NativeEngine,
        evidence: EV_LIB_PIPELINE,
    },
    FrameworkRecord {
        core: "lib",
        framework: "rust",
        status: FrameworkStatus::NativeEngine,
        evidence: "Rust resolver/CAS install path has local E2E; Cargo.lock synchronization for MGC-native add/update is not yet proven, so this record does not claim production-ready parity",
    },
    FrameworkRecord {
        core: "lib",
        framework: "python",
        status: FrameworkStatus::NativeEngine,
        evidence: "Python native path is manifest/artifact constrained; unsupported source/wheel/marker cases must fail closed; per-operation claims are emitted from dep_gate",
    },
    FrameworkRecord {
        core: "lib",
        framework: "go",
        status: FrameworkStatus::NativeEngine,
        evidence: "MGC-native Go dependency lane requires go.mod; no Go scaffold is currently offered by the Lib wizard",
    },
    FrameworkRecord {
        core: "lib",
        framework: "java",
        status: FrameworkStatus::NativeEngine,
        evidence: "MGC-native Maven lane requires pom.xml; Gradle and Java scaffold generation are not currently offered by the Lib wizard",
    },
    FrameworkRecord {
        core: "lib",
        framework: "dotnet",
        status: FrameworkStatus::NativeEngine,
        evidence: "MGC-native NuGet lane requires exactly one root csproj; .NET scaffold generation is not currently offered by the Lib wizard",
    },
];
