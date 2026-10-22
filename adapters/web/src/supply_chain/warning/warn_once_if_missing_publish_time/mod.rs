//! Wiring for the explicit missing-publish-time escape warning.
//! Wiring cảnh báo escape timestamp phát hành bị thiếu.

mod warn_once_if_missing_publish_time;

pub(in crate::supply_chain) use warn_once_if_missing_publish_time::warn_once_if_missing_publish_time;
