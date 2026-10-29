//! Layer-cache owner facade.
//! Facade owner layer cache.

mod entry;
mod error;
mod error_mapping;
mod flow;
mod ref_match;
mod validation;

pub use entry::{ensure_cache_healthy, ensure_layer};
#[cfg(test)]
pub(in crate::commands::template) use error_mapping::template_resolution_error;
#[cfg(test)]
pub(in crate::commands::template) use flow::ensure_layer_in;
#[cfg(test)]
pub(crate) use validation::validate_cached_layer_path_in;
