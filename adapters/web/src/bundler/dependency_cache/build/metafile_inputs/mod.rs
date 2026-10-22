//! Dependency-bundle input provenance wiring.
//! Wiring nguồn gốc input của dependency bundle.

#[path = "document/mod.rs"]
mod document;
#[path = "error/mod.rs"]
mod error;
#[path = "metafile_inputs.rs"]
mod implementation;
#[path = "input_path/mod.rs"]
mod input_path;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(in crate::bundler::dependency_cache) use error::MetafileInputsError;
pub(super) use implementation::validate_metafile_inputs;
