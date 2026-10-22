//! Writes direct root pins and per-owner roots with their established sorting rules.
//! File này chỉ nối owner root_dependencies_section.

mod root_dependencies_section;

pub(crate) use root_dependencies_section::append_root_dependencies_section;
