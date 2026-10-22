//! Optimizer configuration diagnostics.
//! Lỗi cấu hình optimizer.

mod error;

pub use error::{
    optimizer_config_directory_invalid, optimizer_config_missing, optimizer_config_read_failed,
    optimizer_config_setting_invalid, optimizer_env_unmatched_quote,
};
