//! Fullstack Java Web framework metadata.
//! Metadata framework Web fullstack dùng Java.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_SPRING:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-spring",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_QUARKUS:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-quarkus",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_QUARKUS:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-quarkus",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const SVELTE_QUARKUS: WebFrameworkConfig = WebFrameworkConfig {
    name: "svelte-quarkus",
    sub_type: "fullstack",
    base: None,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
