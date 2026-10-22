//! Nearest-owner hardware detection tests.
//! (Kiểm thử parser bộ nhớ đặt cạnh owner parser.)

use crate::commands::optimizer::detect::{HardwareInfo, SystemProfile};

#[test]
fn test_meminfo_parser_fixtures_macos_linux_windows() {
    // Fixture parser RAM đa nền tảng (không gọi subprocess/syscall).
    // (Cross-platform RAM parser fixtures — no subprocess.)
    // macOS sysctl bytes.
    // (Giá trị byte lấy từ sysctl macOS.)
    assert_eq!(
        HardwareInfo::parse_sysctl_memsize_bytes("17179869184\n"),
        Some(17179869184)
    );
    assert_eq!(
        HardwareInfo::parse_sysctl_memsize_bytes("not-a-number\n"),
        None
    );
    assert_eq!(HardwareInfo::parse_sysctl_memsize_bytes(""), None);
    // Linux meminfo.
    // (Dữ liệu meminfo của Linux.)
    let meminfo = "MemTotal:       16384000 kB\nMemFree:         8000000 kB\n";
    assert_eq!(HardwareInfo::parse_meminfo_kb(meminfo), Some(16384000));
    assert_eq!(
        HardwareInfo::parse_meminfo_kb("MemTotal:       2097152 kB\n"),
        Some(2097152)
    );
    assert_eq!(HardwareInfo::parse_meminfo_kb("MemFree: 1 kB\n"), None);
    assert_eq!(HardwareInfo::parse_meminfo_kb(""), None);
    assert_eq!(
        HardwareInfo::parse_meminfo_kb("MemTotal: garbage kB\n"),
        None
    );
    assert_eq!(
        HardwareInfo::parse_cgroup_memory_limit_bytes("8589934592\n"),
        Some(8589934592)
    );
    assert_eq!(HardwareInfo::parse_cgroup_memory_limit_bytes("max\n"), None);
    assert_eq!(HardwareInfo::parse_cgroup_memory_limit_bytes("0\n"), None);
    assert_eq!(
        HardwareInfo::parse_cgroup_memory_limit_bytes("9223372036854771712\n"),
        None
    );
    // Unknown platform RAM degrades without inventing a measurement.
    // (RAM không đọc được thì hạ cấp, không tự tạo số đo.)
    assert_eq!(
        HardwareInfo::profile_for(Some(16), None),
        SystemProfile::Constrained
    );
    assert_eq!(
        HardwareInfo::profile_for(Some(8), Some(16)),
        SystemProfile::HighPerformance
    );
    assert_eq!(
        HardwareInfo::profile_for(Some(4), Some(8)),
        SystemProfile::Standard
    );
    assert_eq!(
        HardwareInfo::profile_for(Some(2), Some(4)),
        SystemProfile::Constrained
    );
}
