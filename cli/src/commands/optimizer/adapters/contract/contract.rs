//! Optimizer adapter contract.
//! Hợp đồng adapter optimizer.

use crate::commands::optimizer::detect::HardwareInfo;
use crate::commands::optimizer::generators::OptimizedConfigFile;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;

/// Trait for runtime-specific optimizer adapters — trait cho adapter tối ưu theo runtime
pub trait OptimizerAdapter {
    /// Human-readable name for this adapter (e.g., "Node.js", "Deno", "PyTorch") — tên dễ đọc cho adapter
    fn name(&self) -> &'static str;

    /// Check if this adapter matches the detected runtime — kiểm tra adapter khớp với runtime đã phát hiện
    fn matches(&self, runtime: &DetectedRuntime) -> bool;

    /// Generate optimization config files for this runtime — tạo các file config tối ưu cho runtime này
    fn generate(&self, hw: &HardwareInfo) -> Vec<OptimizedConfigFile>;
}
