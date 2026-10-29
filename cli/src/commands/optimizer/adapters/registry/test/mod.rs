//! Optimizer adapter registry tests — test danh bạ adapter optimizer.
//! Every language `mgc build` supports must have an adapter; a language that
//! builds but optimizes nothing is the gap these tests exist to prevent.
//! (Mọi ngôn ngữ `mgc build` hỗ trợ phải có adapter; ngôn ngữ build được mà
//! không tối ưu được chính là gap các test này canh.)

use super::all_adapters;
use crate::commands::optimizer::detect::HardwareInfo;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;

fn probe_hardware() -> HardwareInfo {
    // A fixed probe profile so assertions do not depend on the build machine.
    // (Profile probe cố định để assert không phụ thuộc máy build.)
    HardwareInfo {
        cpu_cores: Some(8),
        arch: "aarch64".to_string(),
        os: "linux".to_string(),
        total_memory_gb: Some(16),
        profile: crate::commands::optimizer::detect::SystemProfile::Standard,
        gpus: Vec::new(),
        gpu_detection_status: crate::commands::optimizer::detect::GpuDetectionStatus::Unavailable,
    }
}

/// Each runtime is claimed by exactly one adapter, and that adapter produces
/// at least one configuration file.
/// (Mỗi runtime do đúng một adapter nhận, và adapter đó sinh ít nhất một file.)
#[test]
fn every_supported_runtime_has_exactly_one_adapter() {
    for runtime in [
        DetectedRuntime::RustLib,
        DetectedRuntime::GoLib,
        DetectedRuntime::PythonLib,
        DetectedRuntime::TypeScriptLib,
        DetectedRuntime::JvmLib,
        DetectedRuntime::DotNetLib,
        DetectedRuntime::SwiftLib,
        DetectedRuntime::Flutter,
        DetectedRuntime::ReactNative,
        DetectedRuntime::RustNative,
    ] {
        let adapters = all_adapters();
        let matching: Vec<_> = adapters.iter().filter(|a| a.matches(&runtime)).collect();
        assert_eq!(
            matching.len(),
            1,
            "{runtime:?} must be claimed by exactly one adapter, got {:?}",
            matching.iter().map(|a| a.name()).collect::<Vec<_>>()
        );
        let files = matching[0].generate(&probe_hardware());
        assert!(
            !files.is_empty(),
            "{} advertises {runtime:?} but generates no configuration",
            matching[0].name()
        );
    }
}

/// Adapter names are unique, so a report never shows the same name twice.
/// (Tên adapter không trùng để báo cáo không hiện hai lần cùng tên.)
#[test]
fn adapter_names_are_unique() {
    let adapters = all_adapters();
    let mut names: Vec<&str> = adapters.iter().map(|a| a.name()).collect();
    let total = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), total, "duplicate adapter name");
}

/// Generated paths are unique across adapters: two adapters writing the same
/// file would silently overwrite each other.
/// (Đường dẫn sinh ra không trùng: hai adapter ghi cùng file sẽ ghi đè nhau.)
#[test]
fn generated_paths_do_not_collide() {
    let adapters = all_adapters();
    let hardware = probe_hardware();
    let mut seen = std::collections::BTreeSet::new();
    for adapter in &adapters {
        for file in adapter.generate(&hardware) {
            assert!(
                seen.insert(file.relative_path.clone()),
                "{} writes '{}', already written by another adapter",
                adapter.name(),
                file.relative_path
            );
        }
    }
}

/// The three adapter families that were missing (JVM, .NET, Swift) are real and
/// target their own runtime.
/// (Ba họ adapter từng thiếu — JVM, .NET, Swift — có thật và nhắm đúng runtime.)
#[test]
fn the_newly_covered_runtimes_have_dedicated_adapters() {
    let adapters = all_adapters();
    for (runtime, expected_name) in [
        (DetectedRuntime::JvmLib, "Java Library"),
        (DetectedRuntime::DotNetLib, ".NET Library"),
        (DetectedRuntime::SwiftLib, "Swift Library"),
    ] {
        let found = adapters
            .iter()
            .find(|a| a.matches(&runtime))
            .unwrap_or_else(|| panic!("{runtime:?} has no adapter"));
        assert_eq!(found.name(), expected_name);
    }
}
