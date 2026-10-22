//! Model cache management cho AI adapter.
//! Hỗ trợ HuggingFace cache, Ollama cache, local cache.

pub mod cache_dir;
pub mod cache_size;
pub(super) mod filesystem;
pub mod is_cached;
pub mod list_cached_models;
pub mod metadata;
pub mod model_cache_path;
pub mod prune;

pub use cache_dir::cache_dir;
pub use cache_size::cache_size;
pub use is_cached::is_cached;
pub use list_cached_models::list_cached_models;
pub use model_cache_path::model_cache_path;
