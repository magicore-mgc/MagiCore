//! `cache/mod.rs` — Cache management for lib adapter facade.
//! Facade quản lý cache cho adapter lib (theo khuôn cache web: metadata + prune).

pub mod metadata;
pub mod prune;

mod clear;
mod dir;
mod size;

pub use clear::clear_cache;
pub use dir::cache_dir;
pub use size::cache_size;
