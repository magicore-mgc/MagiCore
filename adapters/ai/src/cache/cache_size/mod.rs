//! Cache size exports — xuất API đo dung lượng cache của registry.

mod cache_size;
mod error;

pub use cache_size::cache_size;

#[cfg(test)]
#[path = "test/cache_size.rs"]
mod tests;
