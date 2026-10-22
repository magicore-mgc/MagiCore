//! AI cache metadata exports — khai báo API metadata cache AI.

mod age;
mod atomic_publish;
mod cache_entry;
mod error;
mod load;
mod save;
mod sidecar_path;

pub use cache_entry::CacheEntry;
use error::MetadataError;
pub use load::load_metadata;
pub use save::save_metadata;
pub(super) use sidecar_path::metadata_path;
