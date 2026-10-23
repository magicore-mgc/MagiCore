//! IoT dependency installation per framework facade.
//! Facade cài dependency IoT theo framework.

mod dispatch;

pub use dispatch::install_dependencies;

#[cfg(test)]
#[path = "test/mod_test.rs"]
mod tests;
