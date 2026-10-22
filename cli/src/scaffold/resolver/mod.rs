//! Cross-core scaffold resolution contracts, split into named owners.
//! Contract phân giải scaffold đa core, tách theo owner có tên.

mod error;
mod missing_layers;
mod resolver;
mod status;

pub use error::ScaffoldResolveError;
pub use missing_layers::MissingLayersReport;
pub use resolver::spec_to_layer_path;
pub use status::ScaffoldResolveStatus;
