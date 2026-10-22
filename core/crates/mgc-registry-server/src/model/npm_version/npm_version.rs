//! npm version document and its nested metadata records.
//! Document version npm và các record metadata đi kèm.

use std::collections::HashMap;

use super::super::optional_deprecation::de_optional_string;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageVersion {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dist: DistInfo,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub peer_dependencies: Option<HashMap<String, String>>,
    pub optional_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<HashMap<String, String>>,
    pub main: Option<String>,
    pub module: Option<String>,
    pub types: Option<String>,
    pub exports: Option<serde_json::Value>,
    pub publish_config: Option<PublishConfig>,
    #[serde(default, deserialize_with = "de_optional_string")]
    pub deprecated: Option<String>,
    pub license: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub author: Option<Author>,
    pub repository: Option<Repository>,
    pub bugs: Option<Bugs>,
    pub homepage: Option<String>,
    pub readme: Option<String>,
    #[serde(rename = "_id", default)]
    pub _id: String,
    #[serde(rename = "_rev", default)]
    pub _rev: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DistInfo {
    pub integrity: String, // sha512 integrity digest — mã integrity sha512
    pub shasum: String,    // sha1 digest — mã băm sha1
    pub tarball: String,   // Tarball URL — địa chỉ tarball
    #[serde(rename = "fileCount", default)]
    pub file_count: Option<u32>,
    #[serde(rename = "unpackedSize", default)]
    pub unpacked_size: Option<u64>,
    #[serde(default)]
    pub signatures: Option<Vec<Signature>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Signature {
    pub keyid: String,
    pub sig: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PublishConfig {
    pub access: Option<String>, // "public" or "restricted" — "public" hoặc "restricted"
    pub registry: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Repository {
    pub r#type: String,
    pub url: String,
    pub directory: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bugs {
    pub url: String,
    pub email: Option<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
