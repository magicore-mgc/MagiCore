//! Metadata load exports — xuất API đọc metadata sidecar.

mod load;

pub use load::load_metadata;

#[cfg(test)]
#[path = "test/load.rs"]
mod tests;
