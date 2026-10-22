//! Fullstack standalone/mixed Web framework metadata.
//! Metadata framework Web fullstack dùng standalone/mixed.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REMIX:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "remix",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const CUSTOM:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "custom",
    sub_type: "fullstack",
    base: None,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
