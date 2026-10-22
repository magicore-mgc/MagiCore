//! Help output owner wiring — wiring cho owner render help.
//! Keeps the public help functions at their existing module path.
//! Giữ các hàm help public tại module path hiện có.

mod help;
mod sections;

pub use help::{custom_help_text, print_custom_help};
