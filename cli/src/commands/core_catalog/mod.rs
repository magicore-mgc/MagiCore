//! Shared catalog for core identity checks.
//! Catalog dùng chung để kiểm tra tên core.

mod core_catalog;

pub(crate) use core_catalog::is_known_core;
