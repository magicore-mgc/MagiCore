//! Hugging Face native client facade / Facade client native Hugging Face.
mod api;
mod artifact;
mod client;
mod error;
mod metadata;

pub use artifact::{download_file, get_model_card};
pub use client::HuggingFaceClient;
pub(in crate::native::hf_client) use error::HuggingFaceClientError;
pub(crate) use metadata::{parse_model_ids, parse_model_info};
