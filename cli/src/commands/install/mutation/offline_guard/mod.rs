//! Offline lock and cache preflight for generic installs.
//! Tiền kiểm lock/cache khi cài offline.

mod offline_guard;

pub(in crate::commands::install) use offline_guard::validate_offline_install;
