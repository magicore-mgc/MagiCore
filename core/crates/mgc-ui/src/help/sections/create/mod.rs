//! Create help section wiring — wiring section help create.
//! Keep this module limited to declaration and export.
//! Giới hạn module này ở khai báo và export.

mod create;
pub(in crate::help) use create::append_create_section;

#[cfg(test)]
#[path = "test/create.rs"]
mod tests;
