//! Nearest-owner hardware detection tests.
//! (Kiểm thử phát hiện phần cứng đặt cạnh owner OS dùng chung.)

use crate::commands::optimizer::detect::{HardwareInfo, SystemProfile};

#[test]
fn test_hardware_detect_returns_valid_info() {
    // Environment-independent shape assertions: arch/os are known; CPU and
    // RAM may be Unknown where the OS does not expose them. Unknown resources
    // must not be converted into fabricated tuning inputs.
    // (CPU/RAM có thể Unknown; không biến giá trị thiếu thành số đo giả.)
    let hw = HardwareInfo::detect();
    assert!(hw.cpu_cores.is_some_and(|cores| cores > 0));
    assert!(!hw.arch.is_empty());
    assert!(!hw.os.is_empty());
    match hw.total_memory_gb {
        Some(gb) => assert!(gb > 0, "detected RAM must be positive, got {gb}"),
        None => assert_eq!(hw.profile, SystemProfile::Constrained),
    }
}
