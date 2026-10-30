//! Cross-core scaffold resolution contracts, split into named owners.
//! Contract phân giải scaffold đa core, tách theo owner có tên.

mod error;
mod missing_layers;
mod remedy;
mod resolver;
mod status;

pub use error::ScaffoldResolveError;
pub use missing_layers::MissingLayersReport;
#[cfg(test)]
pub(crate) use remedy::OFFLINE_GENERATOR_CORES;
pub use resolver::spec_to_layer_path;
pub use status::ScaffoldResolveStatus;
