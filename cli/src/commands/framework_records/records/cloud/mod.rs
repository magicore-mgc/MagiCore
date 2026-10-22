//! Cloud record wiring — nối record Cloud.
#[path = "cloud.rs"]
mod implementation;

pub(super) use self::implementation::RECORDS;
