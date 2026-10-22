//! Fullstack PHP Web framework metadata.
//! Metadata framework Web fullstack dùng PHP.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_LARAVEL:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-laravel",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const REACT_SYMFONY:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "react-symfony",
    sub_type: "fullstack",
    base: None,
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const VUE_SYMFONY:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "vue-symfony",
    sub_type: "fullstack",
    base: None,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
