//! Metadata save exports — xuất API ghi metadata sidecar.

mod save;

pub use save::save_metadata;

#[cfg(test)]
#[path = "test/save.rs"]
mod tests;
