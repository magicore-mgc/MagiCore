//! Web registry configuration owner wiring.
//! Wiring owner cấu hình Web registry.

mod error;
mod web_registry_config;

pub use web_registry_config::{
    DEFAULT_WEB_REGISTRY_URL, WEB_REGISTRY_URL_ENV, advisory_bulk_endpoint, join_registry_path,
    search_endpoint, web_registry_url,
};
