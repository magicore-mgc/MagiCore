//! Backend PHP Web framework metadata.
//! Metadata framework Web backend dùng PHP.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const LARAVEL:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "laravel",
    sub_type: "backend",
    base: Some("php"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const SYMFONY:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "symfony",
    sub_type: "backend",
    base: Some("php"),
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
