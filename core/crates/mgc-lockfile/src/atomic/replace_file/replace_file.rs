//! Compatibility facade for shared platform atomic replacement.
//! Facade tương thích cho primitive thay file atomic của platform.

pub use mgc_platform::atomic_replace::atomic_replace_file;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
