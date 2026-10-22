//! GPU probe dispatch and measured vendor queries.
//! (Điều phối probe GPU và truy vấn hãng dựa trên dữ liệu đã đo.)
use crate::commands::optimizer::detect::{GpuDetectionStatus, GpuInfo, HardwareInfo};

/// Detect GPUs only through a native, in-process OS interface.
/// Linux PCI sysfs is supported; macOS and Windows remain unknown until
/// native IOKit/DXGI implementations are added. No utility process is run.
/// (Hiện chỉ Linux có probe PCI sysfs; macOS/Windows giữ unknown tới khi
/// có IOKit/DXGI native. Không chạy tiến trình tiện ích.)
pub(in crate::commands::optimizer) fn detect_gpus() -> (Vec<GpuInfo>, GpuDetectionStatus) {
    match std::env::consts::OS {
        "linux" => super::linux::detect_gpus_linux(),
        _ => (Vec::new(), GpuDetectionStatus::Unavailable),
    }
}

impl HardwareInfo {
    /// Return vendor presence only for a complete inventory; otherwise unknown.
    /// (`Some(false)` chỉ hợp lệ khi probe đủ; thiếu/partial phải là `None`.)
    pub fn has_gpu_vendor(&self, vendor: &str) -> Option<bool> {
        if self.gpu_detection_status != GpuDetectionStatus::Available {
            return None;
        }
        Some(
            self.gpus
                .iter()
                .any(|gpu| gpu.vendor.as_deref() == Some(vendor)),
        )
    }
}
