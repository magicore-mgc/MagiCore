//! Coordinate safe root-table updates to Cargo.toml.
//! Điều phối cập nhật an toàn các table root Cargo.toml.

mod write_manifest;

pub(super) use self::write_manifest::write_manifest;

#[cfg(test)]
#[path = "test/write_manifest.rs"]
mod tests;
