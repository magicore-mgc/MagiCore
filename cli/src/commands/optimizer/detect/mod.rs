//! Native optimizer hardware detection API facade.
//! (Facade API phát hiện phần cứng gốc cho optimizer.)
//!
//! OS APIs and fixture parsers are separated by responsibility while existing caller paths stay stable.
//! (API hệ điều hành và parser fixture được tách theo trách nhiệm, giữ nguyên các đường dẫn caller.)

mod detect;
mod gpu;
mod memory;
mod profile;
mod types;

pub(super) use gpu::detect_gpus;
pub(super) use memory::detect_memory_gb;
pub use types::{GpuDetectionStatus, GpuInfo, HardwareInfo, SystemProfile};
