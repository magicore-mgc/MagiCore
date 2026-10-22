//! Backend Go Web framework metadata.
//! Metadata framework Web backend dùng Go.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const GIN:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "gin",
    sub_type: "backend",
    base: Some("go"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const ECHO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "echo",
    sub_type: "backend",
    base: Some("go"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const FIBER:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "fiber",
    sub_type: "backend",
    base: Some("go"),
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
