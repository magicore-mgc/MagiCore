//! Web generic record wiring — nối record Web nhóm generic.
#[path = "generic.rs"]
mod implementation;

pub(super) use self::implementation::RECORDS;
