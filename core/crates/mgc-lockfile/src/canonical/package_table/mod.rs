//! Canonical package table writer wiring.
//! File này chỉ export helper ghi bảng package và variant.

mod package_table;

pub(super) use package_table::{push_package_key, push_variant};
