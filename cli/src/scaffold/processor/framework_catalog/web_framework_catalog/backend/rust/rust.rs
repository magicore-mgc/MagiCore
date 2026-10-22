//! Backend Rust Web framework metadata.
//! Metadata framework Web backend dùng Rust.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const AXUM:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "axum",
    sub_type: "backend",
    base: Some("rust"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const ACTIX_WEB:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "actix-web",
    sub_type: "backend",
    base: Some("rust"),
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
