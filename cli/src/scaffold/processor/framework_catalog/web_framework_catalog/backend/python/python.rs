//! Backend Python Web framework metadata.
//! Metadata framework Web backend dùng Python.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const FASTAPI:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "fastapi",
    sub_type: "backend",
    base: Some("python"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const DJANGO:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "django",
    sub_type: "backend",
    base: Some("python"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const FLASK:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "flask",
    sub_type: "backend",
    base: Some("python"),
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
