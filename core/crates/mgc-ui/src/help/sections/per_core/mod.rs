//! Per Core help section wiring — wiring section help per core.
//! Keep this module limited to declaration and export.
//! Giới hạn module này ở khai báo và export.

mod per_core;
pub(in crate::help) use per_core::append_per_core_section;

#[cfg(test)]
#[path = "test/per_core.rs"]
mod tests;
