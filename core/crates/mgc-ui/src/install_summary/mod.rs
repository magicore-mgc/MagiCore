//! Install-summary wiring — wiring cho summary cài đặt.
//! The existing two root functions remain public through re-exports.
//! Hai hàm root hiện tại tiếp tục public qua re-export.

mod install_summary;
#[cfg(test)]
#[path = "test/install_summary.rs"]
mod tests;

pub use install_summary::{print_install_summary, print_install_summary_source};
