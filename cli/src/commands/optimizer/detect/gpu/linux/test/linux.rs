//! Nearest-owner hardware detection tests.
//! (Kiểm thử phát hiện phần cứng đặt cạnh owner GPU Linux.)

use crate::commands::optimizer::detect::{GpuDetectionStatus, HardwareInfo, SystemProfile};
use crate::commands::optimizer::generators::gpu_env_file;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_linux_sysfs_gpu_probe_uses_native_pci_metadata() {
    use std::path::Path;

    let dir = tempdir().unwrap();
    let devices = dir.path().join("bus/pci/devices");
    let nvidia = devices.join("0000:01:00.0");
    let intel = devices.join("0000:00:02.0");
    let network = devices.join("0000:00:1f.6");
    for path in [&nvidia, &intel, &network] {
        fs::create_dir_all(path).unwrap();
    }
    fs::write(nvidia.join("class"), "0x030000\n").unwrap();
    fs::write(nvidia.join("vendor"), "0x10de\n").unwrap();
    fs::write(nvidia.join("device"), "0x2684\n").unwrap();
    fs::write(nvidia.join("product_name"), "NVIDIA GeForce RTX 4090\n").unwrap();
    fs::write(intel.join("class"), "0x030000\n").unwrap();
    fs::write(intel.join("vendor"), "0x8086\n").unwrap();
    fs::write(intel.join("device"), "0x9a49\n").unwrap();
    fs::write(network.join("class"), "0x020000\n").unwrap();
    fs::write(network.join("vendor"), "0x8086\n").unwrap();
    fs::write(network.join("device"), "0x1234\n").unwrap();

    let (detected, status) = HardwareInfo::parse_linux_sysfs_gpus(Path::new(&devices));
    assert_eq!(status, GpuDetectionStatus::Available);
    assert_eq!(
        detected.len(),
        2,
        "non-display PCI devices must be excluded"
    );
    assert!(detected.iter().any(|gpu| {
        gpu.name == "NVIDIA GeForce RTX 4090" && gpu.vendor.as_deref() == Some("nvidia")
    }));
    assert!(detected.iter().any(|gpu| {
        gpu.name == "Intel GPU (PCI 8086:9a49)" && gpu.vendor.as_deref() == Some("intel")
    }));
    assert!(detected.iter().all(|gpu| gpu.vram_mb.is_none()));
}

#[test]
fn test_empty_readable_pci_inventory_is_a_measured_zero() {
    use std::path::Path;

    let dir = tempdir().unwrap();
    let (gpus, status) = HardwareInfo::parse_linux_sysfs_gpus(Path::new(dir.path()));
    assert!(gpus.is_empty());
    assert_eq!(status, GpuDetectionStatus::Available);
}

#[test]
fn test_unreadable_pci_inventory_is_not_reported_as_zero() {
    use std::path::Path;

    let dir = tempdir().unwrap();
    let missing_root = dir.path().join("missing");
    let (gpus, status) = HardwareInfo::parse_linux_sysfs_gpus(Path::new(&missing_root));
    assert!(gpus.is_empty());
    assert_eq!(status, GpuDetectionStatus::Unavailable);
}

#[test]
fn test_incomplete_pci_inventory_does_not_emit_a_zero_count() {
    use std::path::Path;

    let dir = tempdir().unwrap();
    let device = dir.path().join("0000:00:02.0");
    fs::create_dir_all(&device).unwrap();
    fs::write(device.join("class"), "0x030000\n").unwrap();
    fs::write(device.join("vendor"), "malformed\n").unwrap();
    fs::write(device.join("device"), "0x9a49\n").unwrap();

    let (gpus, status) = HardwareInfo::parse_linux_sysfs_gpus(Path::new(dir.path()));
    assert!(gpus.is_empty());
    assert_eq!(status, GpuDetectionStatus::Partial);
    let hw = HardwareInfo {
        cpu_cores: Some(4),
        arch: "x86_64".to_string(),
        os: "linux".to_string(),
        total_memory_gb: Some(8),
        profile: SystemProfile::Standard,
        gpus,
        gpu_detection_status: status,
    };
    assert!(!gpu_env_file(&hw).content.contains("MGC_GPU_COUNT="));
}
