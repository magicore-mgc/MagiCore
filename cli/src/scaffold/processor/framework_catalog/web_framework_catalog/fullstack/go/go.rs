//! Fullstack Go Web framework metadata.
//! Metadata framework Web fullstack dùng Go.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_GIN:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-gin",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_ECHO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-echo",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_FIBER:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-fiber",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_GIN:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-gin",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_ECHO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-echo",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_FIBER:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-fiber",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const SVELTE_GIN:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "svelte-gin",
    sub_type: "fullstack",
    base: None,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
