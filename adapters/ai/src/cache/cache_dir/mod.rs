//! Registry cache-root exports — khai báo owner xác định cache root theo registry.

mod cache_dir;

pub use cache_dir::cache_dir;

#[cfg(test)]
#[path = "test/cache_dir.rs"]
mod tests;
