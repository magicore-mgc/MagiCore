//! Backend Node.js Web framework metadata.
//! Metadata framework Web backend dùng Node.js.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const EXPRESS:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "express",
    sub_type: "backend",
    base: Some("node"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const FASTIFY:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "fastify",
    sub_type: "backend",
    base: Some("node"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const NEST:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "nest",
    sub_type: "backend",
    base: Some("node"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const NESTJS:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "nestjs",
    sub_type: "backend",
    base: Some("node"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const HONO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "hono",
    sub_type: "backend",
    base: Some("node"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const TRPC:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "trpc",
    sub_type: "backend",
    base: Some("node"),
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
