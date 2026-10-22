//! Options help section wiring — wiring section help options.
//! Keep this module limited to declaration and export.
//! Giới hạn module này ở khai báo và export.

mod options;
pub(in crate::help) use options::append_options_section;

#[cfg(test)]
#[path = "test/options.rs"]
mod tests;
