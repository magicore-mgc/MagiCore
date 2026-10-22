//! AI model registry public facade and internal owner wiring.
//! Facade công khai registry AI và nối các owner nội bộ.

pub(super) mod huggingface_query;
pub(super) mod local_query;
pub(super) mod metadata_from_hf_json;
pub mod parser;
mod query_model;
pub(super) mod types;
pub(super) mod unsupported_query;

pub use query_model::query_model;
pub use types::{ModelFormat, ModelMetadata, Registry};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
