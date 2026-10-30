//! Shared RAM-unit conversion and OS probe selection.
//! (Dùng chung phép đổi đơn vị RAM và chọn probe theo hệ điều hành.)
use crate::commands::optimizer::detect::HardwareInfo;

impl HardwareInfo {
    /// Đọc dung lượng RAM từ hệ điều hành
    pub(in crate::commands::optimizer) fn detect_memory_gb() -> Option<usize> {
        #[cfg(target_os = "macos")]
        {
            let bytes = Self::macos_total_memory_bytes()?;
            usize::try_from(bytes / (1024 * 1024 * 1024)).ok()
        }
        #[cfg(target_os = "linux")]
        {
            let host_bytes = std::fs::read_to_string("/proc/meminfo")
                .ok()
                .and_then(|meminfo| Self::parse_meminfo_kb(&meminfo))
                .and_then(|kb| kb.checked_mul(1024));
            let cgroup_limit = Self::linux_cgroup_memory_limit_bytes();
            let effective_bytes = match (host_bytes, cgroup_limit) {
                (Some(host), Some(limit)) => host.min(limit),
                (Some(host), None) => host,
                (None, Some(limit)) => limit,
                (None, None) => return None,
            };
            let gb = effective_bytes / (1024 * 1024 * 1024);
            (gb > 0).then(|| usize::try_from(gb).ok()).flatten()
        }
        #[cfg(target_os = "windows")]
        {
            let bytes = Self::windows_total_memory_bytes()?;
            let gb = bytes / (1024 * 1024 * 1024);
            (gb > 0).then(|| usize::try_from(gb).ok()).flatten()
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            None
        }
    }

    #[cfg(target_os = "macos")]
    fn macos_total_memory_bytes() -> Option<u64> {
        super::os::macos_total_memory_bytes()
    }

    #[cfg(target_os = "windows")]
    fn windows_total_memory_bytes() -> Option<u64> {
        super::os::windows_total_memory_bytes()
    }

    #[cfg(target_os = "linux")]
    fn linux_cgroup_memory_limit_bytes() -> Option<u64> {
        super::os::linux_cgroup_memory_limit_bytes()
    }
}

pub(in crate::commands::optimizer) fn detect_memory_gb() -> Option<usize> {
    HardwareInfo::detect_memory_gb()
}
