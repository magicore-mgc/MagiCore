//! npm registry package document and maintainers.
//! Document package registry npm và maintainers.

use std::collections::HashMap;

use super::super::npm_version::PackageVersion;

/// Package metadata in registry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub name: String,
    pub description: Option<String>,
    pub versions: HashMap<String, PackageVersion>,
    #[serde(rename = "dist-tags", default)]
    pub dist_tags: HashMap<String, String>, // Registry tag mapped to a version — tag registry trỏ đến một version
    #[serde(default)]
    pub maintainers: Vec<Maintainer>,
    /// Standard npm flat timestamp map — Bản đồ thời gian phẳng theo chuẩn npm.
    /// Keys include `created`, `modified`, and version strings — Khóa gồm `created`, `modified` và version.
    #[serde(default)]
    pub time: HashMap<String, String>,
    #[serde(default)]
    pub private: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Maintainer {
    pub name: String,
    pub email: Option<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
