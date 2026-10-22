//! Backend Java Web framework metadata.
//! Metadata framework Web backend dùng Java.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const SPRING_BOOT:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "spring-boot",
    sub_type: "backend",
    base: Some("java"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const QUARKUS:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "quarkus",
    sub_type: "backend",
    base: Some("java"),
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
