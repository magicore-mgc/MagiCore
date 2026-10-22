//! Shared package-reference helpers — helper tham chiếu package dùng chung.

mod package_reference;
pub(in crate::import) use package_reference::{split_name_version, strip_paren_suffix};
