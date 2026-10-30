//! Wiring for one-time supply-chain override warnings.
//! Wiring cảnh báo một lần cho override chuỗi cung ứng.

mod warn_once_if_missing_publish_time;
mod warn_once_if_untrusted;

pub(super) use warn_once_if_missing_publish_time::warn_once_if_missing_publish_time;
pub(super) use warn_once_if_untrusted::warn_each_time_if_untrusted;
