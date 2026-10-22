//! Model cache path exports — khai báo các owner tạo đường dẫn model cache.

mod error;
mod model_cache_path;

#[cfg(test)]
pub(super) use model_cache_path::cache_entry_name;
pub use model_cache_path::model_cache_path;
pub(super) use model_cache_path::model_id_for_cache_entry;

#[cfg(test)]
#[path = "test/model_cache_path.rs"]
mod tests;
