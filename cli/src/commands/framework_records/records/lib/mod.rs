//! Lib record wiring — nối record Lib.
#[path = "lib.rs"]
mod implementation;

pub(super) use self::implementation::RECORDS;
