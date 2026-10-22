//! Unit tests for MagiCore Optimizer Engine

use super::detect::*;
use super::generators::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_generate_optimizations_for_web_core() {
    let dir = tempdir().unwrap();
    let project_root = dir.path();

    // Create package.json to trigger Node.js detection
    fs::write(project_root.join("package.json"), "{}").unwrap();

    let hw = HardwareInfo {
        cpu_cores: Some(8),
        arch: "aarch64".to_string(),
        os: "macos".to_string(),
        total_memory_gb: Some(16),
        profile: SystemProfile::HighPerformance,
        gpus: vec![],
        gpu_detection_status: GpuDetectionStatus::Available,
    };

    let files = generate_optimizations_for_core("web", &hw, project_root);
    // Profile JSON + Node.js Env (from adapter)
    assert!(files.len() >= 2);
    assert!(files[0].relative_path.contains("profile.json"));
    // Node.js adapter should generate config
    assert!(
        files
            .iter()
            .any(|f| f.content.contains("max-old-space-size"))
    );
}

#[test]
fn test_generate_optimizations_for_game_core() {
    let dir = tempdir().unwrap();
    let project_root = dir.path();

    // Create Cargo.toml to trigger Rust detection
    fs::write(
        project_root.join("Cargo.toml"),
        "[package]\nname = \"test\"\n",
    )
    .unwrap();

    let hw = HardwareInfo {
        cpu_cores: Some(12),
        arch: "x86_64".to_string(),
        os: "linux".to_string(),
        total_memory_gb: Some(32),
        profile: SystemProfile::HighPerformance,
        gpus: vec![],
        gpu_detection_status: GpuDetectionStatus::Available,
    };

    let files = generate_optimizations_for_core("game", &hw, project_root);
    assert!(files.len() >= 2);
    // Rust lib adapter should generate Cargo profile config
    assert!(files.iter().any(|f| f.content.contains("jobs")));
}

#[test]
fn test_unknown_ram_skips_memory_derived_configs() {
    // Unknown RAM (None — not Some(0), which would claim a measured
    // zero): only the honest manifest + RAM-independent GPU facts may be
    // emitted, never adapter files with degenerate memory-derived values.
    // (RAM Unknown: chỉ manifest trung thực + fact GPU.)
    let dir = tempdir().unwrap();
    let project_root = dir.path();
    fs::write(project_root.join("package.json"), "{}").unwrap();

    let hw = HardwareInfo {
        cpu_cores: Some(8),
        arch: "x86_64".to_string(),
        os: "linux".to_string(),
        total_memory_gb: None,
        profile: SystemProfile::Constrained,
        gpus: vec![],
        gpu_detection_status: GpuDetectionStatus::Available,
    };

    let files = generate_optimizations_for_core("web", &hw, project_root);
    assert_eq!(files.len(), 2);
    assert!(files[0].relative_path.contains("profile.json"));
    assert!(files[1].relative_path.contains("gpu.env"));
}

#[test]
fn test_optimize_project_without_runtime_fails_closed() {
    // No detectable runtime is NOT a success: automation must observe an
    // error, never Ok(()) for work that never happened.
    // (Không có runtime không phải thành công: automation phải thấy lỗi.)
    let dir = tempdir().unwrap();
    let result = super::optimize_project(dir.path(), "web", false);
    assert!(
        result.is_err(),
        "optimize with no detectable runtime must fail closed"
    );
}

#[test]
fn test_runtime_free_core_profiles_generate_deployment_optimization() {
    let dir = tempdir().unwrap();
    let hw = HardwareInfo {
        cpu_cores: None,
        arch: "unknown".to_string(),
        os: "unknown".to_string(),
        total_memory_gb: None,
        profile: SystemProfile::Constrained,
        gpus: vec![],
        gpu_detection_status: GpuDetectionStatus::Unavailable,
    };

    assert!(has_core_level_optimization("clo"));
    assert!(has_core_level_optimization("cloud"));
    assert!(has_core_level_optimization("cicd"));
    assert!(!has_core_level_optimization("app"));

    let files = generate_optimizations_for_core("cicd", &hw, dir.path());
    let deploy = files
        .iter()
        .find(|file| file.relative_path == ".mgc-optimizer/deploy_concurrency.env")
        .expect("CI/CD profile should be independent of RAM and language runtime");
    assert!(deploy.content.contains("DOCKER_BUILDKIT=1"));
    assert!(deploy.content.contains("CPU count unknown"));
    assert!(!dir.path().join("package.json").exists());
}

#[test]
fn test_runtime_free_core_optimization_completes_without_runtime() {
    let dir = tempdir().unwrap();
    super::optimize_project(dir.path(), "cicd", false).unwrap();
    assert!(
        dir.path()
            .join(".mgc-optimizer/deploy_concurrency.env")
            .is_file()
    );
}

#[test]
fn test_hash_guard_prevents_overwriting_user_custom_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let opt_file = OptimizedConfigFile {
        relative_path: ".mgc-optimizer/test.env".to_string(),
        content: "# Generated by MagiCore Optimizer\nKEY=VALUE\n".to_string(),
        description: "Test env".to_string(),
    };

    // Lần 1: Tạo mới thành công
    let ok = apply_optimized_file(root, &opt_file, false).unwrap();
    assert!(ok);

    // User sửa tay file (mất comment Generated by)
    let file_path = root.join(".mgc-optimizer/test.env");
    fs::write(&file_path, "USER_CUSTOM_KEY=CUSTOM_VALUE\n").unwrap();

    // Lần 2 (force=false): Phải từ chối ghi đè để bảo vệ chỉnh sửa của user
    let ok2 = apply_optimized_file(root, &opt_file, false).unwrap();
    assert!(!ok2);
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "USER_CUSTOM_KEY=CUSTOM_VALUE\n");

    // Lần 3 (force=true): Bắt buộc ghi đè
    let ok3 = apply_optimized_file(root, &opt_file, true).unwrap();
    assert!(ok3);
    let content_after = fs::read_to_string(&file_path).unwrap();
    assert!(content_after.contains("Generated by MagiCore Optimizer"));
}

#[test]
fn test_gpu_env_file_zero_and_measured() {
    use super::detect::{GpuDetectionStatus, GpuInfo, HardwareInfo, SystemProfile};
    use super::generators::gpu_env_file;
    // Complete inventory with no GPUs is a measured zero.
    let hw = HardwareInfo {
        cpu_cores: Some(4),
        arch: "x86_64".to_string(),
        os: "linux".to_string(),
        total_memory_gb: Some(8),
        profile: SystemProfile::Standard,
        gpus: vec![],
        gpu_detection_status: GpuDetectionStatus::Available,
    };
    let file = gpu_env_file(&hw);
    assert_eq!(file.relative_path, ".mgc-optimizer/gpu.env");
    assert!(file.content.contains("MGC_GPU_COUNT=0"));
    // GPU đo được — fact đầy đủ, VRAM None ghi rõ lý do
    let hw = HardwareInfo {
        gpus: vec![
            GpuInfo {
                name: "Apple M2".to_string(),
                vendor: Some("apple".to_string()),
                vram_mb: None,
            },
            GpuInfo {
                name: "NVIDIA GeForce RTX 4090".to_string(),
                vendor: Some("nvidia".to_string()),
                vram_mb: Some(24564),
            },
        ],
        ..hw
    };
    let file = gpu_env_file(&hw);
    assert!(file.content.contains("MGC_GPU_COUNT=2"));
    assert!(file.content.contains("MGC_GPU_0_NAME=Apple M2"));
    assert!(file.content.contains("MGC_GPU_0_VENDOR=apple"));
    assert!(file.content.contains("unified memory or unknown"));
    assert!(file.content.contains("MGC_GPU_1_VRAM_MB=24564"));
}

#[test]
fn test_unknown_gpu_probe_does_not_emit_a_fake_zero_count() {
    use super::detect::{GpuDetectionStatus, HardwareInfo, SystemProfile};
    use super::generators::gpu_env_file;

    let hw = HardwareInfo {
        cpu_cores: Some(4),
        arch: "aarch64".to_string(),
        os: "macos".to_string(),
        total_memory_gb: Some(16),
        profile: SystemProfile::Standard,
        gpus: vec![],
        gpu_detection_status: GpuDetectionStatus::Unavailable,
    };
    let output = gpu_env_file(&hw);

    assert!(!output.content.contains("MGC_GPU_COUNT="));
    assert!(output.content.contains("GPU detection unavailable"));
}

#[test]
fn test_gpu_env_sanitizes_control_characters_without_key_injection() {
    let hw = HardwareInfo {
        cpu_cores: Some(4),
        arch: "x86_64".to_string(),
        os: "linux".to_string(),
        total_memory_gb: Some(8),
        profile: SystemProfile::Standard,
        gpus: vec![GpuInfo {
            name: "Display\nMGC_GPU_COUNT=999".to_string(),
            vendor: Some("nvidia\r\nUNEXPECTED=1".to_string()),
            vram_mb: None,
        }],
        gpu_detection_status: GpuDetectionStatus::Available,
    };

    let output = gpu_env_file(&hw);
    assert!(output.content.contains("MGC_GPU_COUNT=1\n"));
    assert!(
        output
            .content
            .contains("MGC_GPU_0_NAME=Display MGC_GPU_COUNT=999\n")
    );
    assert!(
        output
            .content
            .contains("MGC_GPU_0_VENDOR=nvidia  UNEXPECTED=1\n")
    );
    assert!(!output.content.contains("\nUNEXPECTED=1\n"));
}

#[test]
fn test_unknown_ram_omits_memory_knobs_per_adapter() {
    // Từng adapter bỏ knob dẫn xuất từ RAM khi Unknown (không số bịa).
    // (Each adapter omits its memory knob when RAM is Unknown.)
    use super::adapters::OptimizerAdapter;
    use super::adapters::{flutter::FlutterAdapter, pytorch::PyTorchAdapter};
    let hw = HardwareInfo {
        cpu_cores: Some(8),
        arch: "x86_64".to_string(),
        os: "linux".to_string(),
        total_memory_gb: None,
        profile: SystemProfile::Constrained,
        gpus: vec![],
        gpu_detection_status: GpuDetectionStatus::Available,
    };
    let known = HardwareInfo {
        total_memory_gb: Some(16),
        ..hw.clone()
    };
    let flutter_unknown = FlutterAdapter.generate(&hw);
    let flutter_known = FlutterAdapter.generate(&known);
    assert!(
        !flutter_unknown
            .iter()
            .any(|file| file.content.contains("old_gen_heap_size")),
        "unknown RAM must not emit a heap size"
    );
    assert!(
        flutter_known
            .iter()
            .any(|file| file.content.contains("old_gen_heap_size=4096")),
        "known 16GB RAM must emit the derived heap"
    );
    let pytorch_unknown = PyTorchAdapter.generate(&hw);
    let pytorch_known = PyTorchAdapter.generate(&known);
    assert!(
        !pytorch_unknown
            .iter()
            .any(|file| file.content.contains("CONTAINER_MEMORY")),
        "unknown RAM must not emit a container memory limit"
    );
    assert!(
        pytorch_known
            .iter()
            .any(|file| file.content.contains("CONTAINER_MEMORY=14g")),
        "known 16GB RAM must emit the derived limit"
    );
}
