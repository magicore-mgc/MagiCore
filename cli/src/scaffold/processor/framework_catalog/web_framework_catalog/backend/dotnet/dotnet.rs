//! Backend .NET Web framework metadata.
//! Metadata framework Web backend dùng .NET.

use super::super::super::types::WebFrameworkConfig;
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const DOTNET_MINIMAL: WebFrameworkConfig = WebFrameworkConfig {
    name: "dotnet-minimal",
    sub_type: "backend",
    base: Some("dotnet"),
};
pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) const DOTNET_WEBAPI:
    WebFrameworkConfig = WebFrameworkConfig {
    name: "dotnet-webapi",
    sub_type: "backend",
    base: Some("dotnet"),
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
