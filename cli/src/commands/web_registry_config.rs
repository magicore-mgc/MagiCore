//! Public Web registry configuration facade.
//! Facade cấu hình Web registry public.

mod web_registry_config;

pub use web_registry_config::{
    DEFAULT_WEB_REGISTRY_URL, WEB_REGISTRY_URL_ENV, advisory_bulk_endpoint, join_registry_path,
    search_endpoint, web_registry_url,
};
