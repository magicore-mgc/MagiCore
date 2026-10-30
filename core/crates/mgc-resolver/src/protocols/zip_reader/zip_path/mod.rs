//! ZIP path sanitizer facade — wiring cho sanitizer path ZIP.

mod zip_path;

pub(super) use zip_path::sanitize_zip_path;

#[cfg(test)]
#[path = "test/zip_path.rs"]
mod tests;
