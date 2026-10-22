//! Tarball packing coordinator exports — export coordinator đóng gói.

mod pack;

pub use pack::pack;

#[cfg(test)]
#[path = "test/pack.rs"]
mod tests;
