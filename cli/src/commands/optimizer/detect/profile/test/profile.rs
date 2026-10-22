//! Nearest-owner hardware detection tests.
//! (Kiểm thử chọn hồ sơ đặt cạnh owner profile.)

use crate::commands::optimizer::detect::{HardwareInfo, SystemProfile};

#[test]
fn test_profile_for_unknown_ram_degrades_to_constrained() {
    // Unknown RAM must never tune from a guessed size: even a 64-core
    // machine degrades to Constrained when memory cannot be measured.
    // (RAM unknown không bao giờ tuning từ số đoán: máy 64 core cũng hạ
    // về Constrained khi không đo được RAM.)
    assert_eq!(
        HardwareInfo::profile_for(Some(64), None),
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

#[test]
fn test_profile_for_unknown_cpu_degrades_to_constrained() {
    assert_eq!(
        HardwareInfo::profile_for(None, Some(128)),
        SystemProfile::Constrained
    );
}
