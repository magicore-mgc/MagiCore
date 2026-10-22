//! HTTP proxy configuration facade.
//! Facade cấu hình proxy HTTP.

mod apply;
mod bypass;
mod environment;
mod error;
mod proxy_config;
mod proxy_selection;
mod registry_override;

pub use proxy_config::ProxyConfig;

#[cfg(test)]
#[path = "test/support.rs"]
mod test_support;
