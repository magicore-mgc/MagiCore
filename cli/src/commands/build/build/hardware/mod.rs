#[path = "error.rs"]
mod error;
use error::HardwareBuildError;

#[path = "hardware.rs"]
mod implementation;

pub(super) use implementation::build_hardware;
