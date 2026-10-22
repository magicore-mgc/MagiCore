//! Fullstack Python Web framework metadata.
//! Metadata framework Web fullstack dùng Python.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_FASTAPI:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-fastapi",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_DJANGO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-django",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_FLASK:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-flask",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_FASTAPI:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-fastapi",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_DJANGO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-django",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_FLASK:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-flask",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const SVELTE_FASTAPI: WebFrameworkConfig = WebFrameworkConfig {
    name: "svelte-fastapi",
    sub_type: "fullstack",
    base: None,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
