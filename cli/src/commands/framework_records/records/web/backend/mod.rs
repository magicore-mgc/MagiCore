//! Web backend record wiring — nối record Web nhóm backend.
#[path = "backend.rs"]
mod implementation;

pub(super) use self::implementation::RECORDS;
