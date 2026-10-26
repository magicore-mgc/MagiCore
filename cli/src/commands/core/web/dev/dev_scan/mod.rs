//! Dev source scan owner.
//! Owner scan source dev.

#[path = "dev_scan.rs"]
mod implementation;

pub(crate) use implementation::scan_sources;
