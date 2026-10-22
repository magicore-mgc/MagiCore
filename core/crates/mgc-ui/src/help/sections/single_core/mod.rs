//! Single Core help section wiring — wiring section help single core.
//! Keep this module limited to declaration and export.
//! Giới hạn module này ở khai báo và export.

mod single_core;
pub(in crate::help) use single_core::append_single_core_section;

#[cfg(test)]
#[path = "test/single_core.rs"]
mod tests;
