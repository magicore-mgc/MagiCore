//! Pure CPU/RAM profile selection.
//! (Chọn hồ sơ thuần từ số CPU và dung lượng RAM.)
use crate::commands::optimizer::detect::{HardwareInfo, SystemProfile};

impl HardwareInfo {
    /// Pure profile selection — unknown CPU or RAM always degrades to
    /// Constrained; no profile is derived from a guessed resource count.
    /// (CPU hoặc RAM chưa biết thì hạ về Constrained, không đoán tài nguyên.)
    pub fn profile_for(cpu_cores: Option<usize>, memory_gb: Option<usize>) -> SystemProfile {
        let (Some(cpu_cores), Some(total_memory_gb)) = (cpu_cores, memory_gb) else {
            return SystemProfile::Constrained;
        };
        if cpu_cores >= 8 && total_memory_gb >= 16 {
            SystemProfile::HighPerformance
        } else if cpu_cores >= 4 && total_memory_gb >= 8 {
            SystemProfile::Standard
        } else {
            SystemProfile::Constrained
        }
    }
}
