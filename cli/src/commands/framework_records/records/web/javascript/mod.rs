//! Web javascript record wiring — nối record Web nhóm javascript.
#[path = "javascript.rs"]
mod implementation;

pub(super) use self::implementation::RECORDS;
