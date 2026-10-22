//! Manifest I/O owner wiring.
//! Wiring cho owner manifest I/O.

#[path = "manifest_io.rs"]
mod manifest_io;
pub use manifest_io::parse_manifest_uses_dir_name;
