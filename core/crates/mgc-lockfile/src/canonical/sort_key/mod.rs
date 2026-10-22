//! Canonical sort-key owner wiring.
//! File này chỉ export các khóa sort canonical.

mod sort_key;

pub(super) use sort_key::{edge_sort_key, package_sort_key};
