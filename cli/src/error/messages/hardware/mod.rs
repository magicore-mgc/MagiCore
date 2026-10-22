//! Hardware target and device diagnostics.
//! Lỗi target phần cứng và thiết bị.

mod error;

pub use error::{
    android_device_command_failed, espflash_failed, flash_framework_unsupported, fw_path_not_utf8,
    hardware_template_not_materialized, hardware_template_remove_failed, no_board_specified,
    no_hardware_framework, unknown_hardware_framework, unknown_hardware_package, unsupported_board,
};
