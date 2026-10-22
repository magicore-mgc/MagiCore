//! Fullstack Rust Web framework metadata.
//! Metadata framework Web fullstack dùng Rust.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_AXUM:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-axum",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_ACTIX_WEB: WebFrameworkConfig = WebFrameworkConfig {
    name: "react-actix-web",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_AXUM:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-axum",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_ACTIX_WEB:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-actix-web",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const SVELTE_AXUM:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "svelte-axum",
    sub_type: "fullstack",
    base: None,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
