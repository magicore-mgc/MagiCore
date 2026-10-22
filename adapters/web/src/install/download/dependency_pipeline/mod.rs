//! Web dependency download/extraction pipeline wiring.
//! Wiring pipeline tải và giải nén dependency Web.

mod dependency_pipeline;
mod error;
mod result_aggregation;

pub(super) use error::PipelineError;

pub use dependency_pipeline::pipeline_download_and_extract;
