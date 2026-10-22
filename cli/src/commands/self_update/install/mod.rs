//! Declare binary replacement and exact-version probe owners.
//! Khai báo owner thay binary và probe version chính xác.

mod install;
mod version_probe;
pub(in crate::commands) use install::swap_binary;
pub(in crate::commands) use version_probe::probe_reports_version;
