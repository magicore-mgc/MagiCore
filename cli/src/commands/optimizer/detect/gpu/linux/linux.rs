//! Linux PCI sysfs GPU probe and fixture parser.
//! (Probe GPU PCI qua sysfs Linux và bộ phân tích fixture tương ứng.)
use crate::commands::optimizer::detect::{GpuDetectionStatus, GpuInfo, HardwareInfo};
use std::path::Path;

/// Linux: enumerate PCI display controllers from kernel sysfs metadata.
/// (Linux: liệt kê display controller qua metadata sysfs của kernel.)
pub(in crate::commands::optimizer::detect::gpu) fn detect_gpus_linux()
-> (Vec<GpuInfo>, GpuDetectionStatus) {
    HardwareInfo::parse_linux_sysfs_gpus(Path::new("/sys/bus/pci/devices"))
}

impl HardwareInfo {
    /// Parse kernel PCI sysfs entries; caller supplies root for hermetic tests.
    /// (Đọc entry PCI sysfs; nhận root để test fixture không phụ thuộc máy.)
    pub(crate) fn parse_linux_sysfs_gpus(root: &Path) -> (Vec<GpuInfo>, GpuDetectionStatus) {
        let Ok(entries) = std::fs::read_dir(root) else {
            return (Vec::new(), GpuDetectionStatus::Unavailable);
        };
        let mut paths = Vec::new();
        let mut status = GpuDetectionStatus::Available;
        for entry in entries {
            match entry {
                Ok(entry) => paths.push(entry.path()),
                Err(_) => status = GpuDetectionStatus::Partial,
            }
        }
        paths.sort();
        let mut gpus = Vec::new();
        for device_path in paths {
            let class = match std::fs::read_to_string(device_path.join("class")) {
                Ok(class) => class,
                Err(_) => {
                    status = GpuDetectionStatus::Partial;
                    continue;
                }
            };
            let Ok(class) = u32::from_str_radix(class.trim().trim_start_matches("0x"), 16) else {
                status = GpuDetectionStatus::Partial;
                continue;
            };
            if class >> 16 != 0x03 {
                continue;
            }
            let Some(vendor_id) = read_pci_id(&device_path.join("vendor")) else {
                status = GpuDetectionStatus::Partial;
                continue;
            };
            let Some(device_id) = read_pci_id(&device_path.join("device")) else {
                status = GpuDetectionStatus::Partial;
                continue;
            };
            let vendor_name = match vendor_id {
                0x10de => Some("nvidia"),
                0x1002 => Some("amd"),
                0x8086 => Some("intel"),
                0x106b => Some("apple"),
                _ => None,
            };
            let product_name = std::fs::read_to_string(device_path.join("product_name"))
                .or_else(|_| std::fs::read_to_string(device_path.join("label")))
                .ok()
                .map(|name| name.trim().to_string())
                .filter(|name| {
                    !name.is_empty() && name.len() <= 256 && !name.chars().any(char::is_control)
                });
            let name = product_name.unwrap_or_else(|| match vendor_name {
                Some("nvidia") => format!("NVIDIA GPU (PCI {vendor_id:04x}:{device_id:04x})"),
                Some("amd") => format!("AMD GPU (PCI {vendor_id:04x}:{device_id:04x})"),
                Some("intel") => format!("Intel GPU (PCI {vendor_id:04x}:{device_id:04x})"),
                Some("apple") => format!("Apple GPU (PCI {vendor_id:04x}:{device_id:04x})"),
                _ => format!("PCI display adapter ({vendor_id:04x}:{device_id:04x})"),
            });
            gpus.push(GpuInfo {
                name,
                vendor: vendor_name.map(str::to_string),
                vram_mb: None,
            });
        }
        (gpus, status)
    }
}

fn read_pci_id(path: &Path) -> Option<u16> {
    let value = std::fs::read_to_string(path).ok()?;
    u16::from_str_radix(value.trim().trim_start_matches("0x"), 16).ok()
}
