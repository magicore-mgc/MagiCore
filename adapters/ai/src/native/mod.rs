//! Native API clients cho AI registries.

pub mod hf_client;

mod contract;

pub use contract::{ApiClient, ModelInfo};
pub use hf_client::HuggingFaceClient;

#[cfg(test)]
#[path = "test/native_tests.rs"]
mod tests;
