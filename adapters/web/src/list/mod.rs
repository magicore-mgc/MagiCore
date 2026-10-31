//! Web installed-package listing owner.
//! Owner liệt kê package đã cài của Web.

mod list;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub use list::run_list;
