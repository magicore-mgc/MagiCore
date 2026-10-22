// Scaffold writer wiring — nối các writer theo loại output.
// Each output owner keeps core, Web, and common files separate.
use super::*;
#[path = "common_files/mod.rs"]
mod common_files;
#[path = "core_files/mod.rs"]
mod core_files;
#[path = "web_files/mod.rs"]
mod web_files;
