//! Engine binaries cache management facade.
//! Facade quản lý cache binary engine (Godot/Unity/Unreal trong MGC_DATA_DIR/cache/game/).

mod binary;
mod dir;
mod listing;

pub use binary::{cached_binary_path, is_cached, remove_cached_binary};
pub use dir::cache_dir;
pub use listing::list_cached_versions;

#[cfg(test)]
#[path = "test/mod_test.rs"]
mod tests;
