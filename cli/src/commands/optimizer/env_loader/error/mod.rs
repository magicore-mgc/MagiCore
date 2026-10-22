//! Local typed diagnostics for optimizer environment loading.
//! (Lỗi typed cục bộ cho luồng nạp environment của optimizer.)

mod error;

#[cfg(test)]
pub(super) use error::OptimizerEnvError;
pub(super) use error::{
    config_missing, config_read_failed, directory_invalid, setting_invalid, unmatched_quote,
};

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
