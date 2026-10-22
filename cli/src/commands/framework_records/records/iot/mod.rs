//! IoT record wiring — nối record IoT.
#[path = "iot.rs"]
mod implementation;

pub(super) use self::implementation::RECORDS;
