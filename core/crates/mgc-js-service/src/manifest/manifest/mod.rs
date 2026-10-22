//! Manifest conversion owner exports.
//! Export owner chuyển đổi manifest.

mod manifest;

pub use manifest::{parse_manifest, write_manifest};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
