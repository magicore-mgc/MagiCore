//! Common help section wiring — wiring section help common.
//! Keep this module limited to declaration and export.
//! Giới hạn module này ở khai báo và export.

mod common;
pub(in crate::help) use common::append_common_section;

#[cfg(test)]
#[path = "test/common.rs"]
mod tests;
