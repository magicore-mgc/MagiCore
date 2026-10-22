//! Fullstack Node.js Web framework metadata.
//! Metadata framework Web fullstack dùng Node.js.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_FASTIFY:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-fastify",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_EXPRESS:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-express",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_HONO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-hono",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_NESTJS:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-nestjs",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_TRPC:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-trpc",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_EXPRESS:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-express",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_HONO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-hono",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_NESTJS:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-nestjs",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const SVELTE_EXPRESS: WebFrameworkConfig = WebFrameworkConfig {
    name: "svelte-express",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const SVELTE_HONO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "svelte-hono",
    sub_type: "fullstack",
    base: None,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
