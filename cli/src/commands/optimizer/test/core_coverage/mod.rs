// Cores the CLI advertises. Kept as the exhaustive list so a new core must add
// itself here — and therefore to the optimizer — or this test fails.
// (Core mà CLI quảng cáo. Giữ làm danh sách đầy đủ: thêm core mới phải thêm ở
// đây — và do đó vào optimizer — nếu không test này fail.)
const ADVERTISED_CORES: &[&str] = &[
    "web", "ai", "app", "lib", "game", "iot", "clo", "cicd", "hardware",
];

#[test]
fn every_advertised_core_produces_at_least_the_machine_level_profile() {
    use crate::commands::optimizer::detect::{GpuDetectionStatus, HardwareInfo, SystemProfile};
    use crate::commands::optimizer::generators::generate_optimizations_for_core;

    let hw = HardwareInfo {
        cpu_cores: Some(8),
        arch: "aarch64".to_string(),
        os: "linux".to_string(),
        total_memory_gb: Some(16),
        profile: SystemProfile::Standard,
        gpus: Vec::new(),
        gpu_detection_status: GpuDetectionStatus::Unavailable,
    };
    let root = tempfile::tempdir().expect("temp dir");

    for core in ADVERTISED_CORES {
        let files = generate_optimizations_for_core(core, &hw, root.path());
        let paths: Vec<&str> = files.iter().map(|f| f.relative_path.as_str()).collect();
        assert!(
            paths.contains(&".mgc-optimizer/profile.json"),
            "{core}: the hardware profile is a property of the machine and applies \
             to every core, yet it is missing — `mgc optimizer` would report \
             'nothing was optimized' for this core"
        );
        assert!(
            !files.is_empty(),
            "{core}: no configuration generated at all"
        );
    }
}

/// A directory that looks like a project of `core` must select at least one
/// runtime, so `mgc optimizer` has a lane to generate for.
///
/// The fixture is the marker each core's own scaffold writes. A core with no
/// branch in `detect_runtimes` falls into the generic lane, which still counts —
/// the property asserted is "an answer, not an empty refusal".
/// (Thư mục trông như project của `core` phải chọn được ít nhất một runtime.
/// Fixture là marker mà scaffolder của core đó ghi. Core không có nhánh
/// `detect_runtimes` rơi vào lane chung — vẫn tính, vì thuộc tính cần kiểm là
/// "có câu trả lời", không phải "từ chối rỗng".)
#[test]
fn every_advertised_core_selects_a_runtime_for_its_own_project() {
    use crate::commands::optimizer::runtime_detect::{DetectedRuntime, detect_runtimes};

    // (core, the manifest its scaffold writes)
    // (core, manifest mà scaffolder của nó ghi.)
    let fixtures: &[(&str, &str, &str)] = &[
        (
            "web",
            "package.json",
            "{\"name\":\"w\",\"dependencies\":{\"react\":\"19\"}}",
        ),
        ("ai", "pyproject.toml", "[project]\nname = \"a\"\n"),
        ("app", "Package.swift", "// swift-tools-version:5.9\n"),
        ("lib", "go.mod", "module x\n"),
        ("game", "project.godot", "config_version=5\n"),
        ("iot", "platformio.ini", "[env]\n"),
        ("clo", "main.tf", "terraform {}\n"),
        ("cicd", "mgc.toml", "name = \"ci\"\necosystem = \"cicd\"\n"),
        (
            "hardware",
            "mgc.toml",
            "name = \"h\"\necosystem = \"hardware\"\n",
        ),
    ];
    assert_eq!(
        fixtures.len(),
        ADVERTISED_CORES.len(),
        "every advertised core needs a fixture, or this test silently skips it"
    );

    for (core, manifest, body) in fixtures {
        let dir = tempfile::tempdir().expect("temp dir");
        if let Some(parent) = std::path::Path::new(manifest).parent() {
            std::fs::create_dir_all(dir.path().join(parent)).expect("mkdir");
        }
        std::fs::write(dir.path().join(manifest), body).expect("write fixture");

        let runtimes = detect_runtimes(dir.path(), core);
        // `detect_runtimes` never returns an empty vec: when nothing matches it
        // returns `[Unknown]`, and `Unknown` is exactly the value that makes
        // `mgc optimizer` refuse. Asserting `!is_empty()` therefore passed even
        // with the detection branch deleted — this assertion is the one that
        // actually holds the property.
        // (`detect_runtimes` không bao giờ rỗng: khi không khớp nó trả `[Unknown]`,
        // và `Unknown` chính là giá trị khiến optimizer từ chối. Vì vậy assert
        // `!is_empty()` vẫn pass dù xoá nhánh — chỉ assert này giữ đúng thuộc tính.)
        assert!(
            !runtimes.contains(&DetectedRuntime::Unknown),
            "{core}: a project with '{manifest}' resolved to Unknown — `mgc optimizer` \
             would refuse with 'nothing was optimized'"
        );
    }
}
