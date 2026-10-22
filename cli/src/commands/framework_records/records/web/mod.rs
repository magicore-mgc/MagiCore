//! Web record family wiring — nối các nhóm record Web.
#[path = "backend/mod.rs"]
mod backend;
#[path = "generic/mod.rs"]
mod generic;
#[path = "web.rs"]
mod implementation;
#[path = "javascript/mod.rs"]
mod javascript;

pub(super) use self::implementation::RECORDS;
