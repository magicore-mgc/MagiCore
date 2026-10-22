//! App record wiring — nối record App.
#[path = "app.rs"]
mod implementation;

pub(super) use self::implementation::RECORDS;
