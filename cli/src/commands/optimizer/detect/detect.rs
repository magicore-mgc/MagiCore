//! Hardware snapshot assembly from native, in-process OS probes.
//! (Ghép snapshot phần cứng từ các probe hệ điều hành chạy trực tiếp trong tiến trình.)
use super::HardwareInfo;

impl HardwareInfo {
    /// Tự động phát hiện thông số phần cứng từ môi trường runtime
    pub fn detect() -> Self {
        let cpu_cores = std::thread::available_parallelism().ok().map(|n| n.get());
        let arch = std::env::consts::ARCH.to_string();
        let os = std::env::consts::OS.to_string();

        // RAM detection failure stays unknown (None) — never patched
        // with a fabricated number; the profile degrades to Constrained
        // and memory-derived tuning is skipped downstream. Writing tuned
        // values from a guessed size would be false measurement.
        // (Đọc RAM thất bại giữ None — không vá số bịa.)
        let memory_gb = super::detect_memory_gb();
        let total_memory_gb = memory_gb;
        let profile = Self::profile_for(cpu_cores, memory_gb);
        let (gpus, gpu_detection_status) = super::detect_gpus();

        Self {
            cpu_cores,
            arch,
            os,
            total_memory_gb,
            profile,
            gpus,
            gpu_detection_status,
        }
    }
}
