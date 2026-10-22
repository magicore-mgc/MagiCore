//! Pure, fixture-driven parsers for platform memory metadata.
//! (Parser thuần dựa trên fixture cho metadata bộ nhớ các nền tảng.)
use crate::commands::optimizer::detect::HardwareInfo;

impl HardwareInfo {
    /// Parse cgroup v1/v2 memory limit; `max` and huge v1 sentinels mean unlimited.
    /// (Đọc giới hạn cgroup v1/v2; `max` và sentinel lớn nghĩa là không giới hạn.)
    #[cfg(any(target_os = "linux", test))]
    pub(crate) fn parse_cgroup_memory_limit_bytes(text: &str) -> Option<u64> {
        let value = text.trim().parse::<u64>().ok()?;
        (value > 0 && value < (1_u64 << 60)).then_some(value)
    }

    /// Parse a decimal byte count from the macOS sysctl value — fixture-tested.
    /// (Parse số byte thập phân từ sysctl macOS — có fixture test.)
    #[cfg(test)]
    pub(crate) fn parse_sysctl_memsize_bytes(text: &str) -> Option<u64> {
        text.trim().parse().ok()
    }

    /// Parse Linux `/proc/meminfo` MemTotal line (kB) — pure,
    /// fixture-tested on every CI OS (the live caller is Linux-only).
    /// (Parse MemTotal meminfo — thuần, test bằng fixture mọi OS.)
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    pub(crate) fn parse_meminfo_kb(text: &str) -> Option<u64> {
        for line in text.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let kb: u64 = parts[1].parse().ok()?;
                    return Some(kb);
                }
            }
        }
        None
    }
}
