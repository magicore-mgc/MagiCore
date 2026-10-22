#[path = "error.rs"]
mod error;
use error::IotBuildError;

#[path = "iot.rs"]
mod implementation;

pub(super) use implementation::build_iot;
