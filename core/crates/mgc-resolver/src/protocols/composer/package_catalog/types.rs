//! Packagist p2 metadata shapes (Composer v2 repository document).
//! Cấu trúc metadata p2 của Packagist (tài liệu repository Composer v2).

use serde::Deserialize;
use std::collections::BTreeMap;

/// Composer's minified metadata replaces a cleared field with this sentinel
/// string instead of omitting it.
/// Metadata minified của Composer thay field đã xoá bằng chuỗi sentinel này
/// thay vì bỏ hẳn.
const UNSET_SENTINEL: &str = "__unset";

/// `require` / `require-dev` may be a map, `null`, or `"__unset"`.
/// `require` / `require-dev` có thể là map, `null` hoặc `"__unset"`.
fn dependency_map<'de, D>(deserializer: D) -> Result<BTreeMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(BTreeMap::new()),
        serde_json::Value::String(sentinel) if sentinel == UNSET_SENTINEL => Ok(BTreeMap::new()),
        serde_json::Value::Object(map) => map
            .into_iter()
            .map(|(key, value)| {
                let value = value.as_str().map(str::to_string).unwrap_or_else(|| {
                    // Non-string constraints are not Composer syntax; keep a
                    // readable placeholder so the caller fails loudly.
                    // Constraint không phải chuỗi không đúng cú pháp Composer;
                    // giữ placeholder để caller báo lỗi rõ.
                    value.to_string()
                });
                Ok((key, value))
            })
            .collect(),
        other => Err(serde::de::Error::custom(format!(
            "unexpected Composer dependency section: {other}"
        ))),
    }
}

/// `dist` may be a map, `null`, or `"__unset"`.
/// `dist` có thể là map, `null` hoặc `"__unset"`.
fn optional_dist<'de, D>(deserializer: D) -> Result<Option<PackagistDist>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::String(sentinel) if sentinel == UNSET_SENTINEL => Ok(None),
        other => serde_json::from_value(other)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

/// Composer v2 repository document: `{"minified":..,"packages":{..}}`.
/// Field thừa của Composer (autoload, authors, funding…) vẫn qua được vì
/// serde chỉ đọc những khoá được khai báo.
#[derive(Debug, Deserialize)]
pub(in crate::protocols::composer) struct PackagistDocument {
    /// Package lists keyed by lowercase `vendor/package`.
    /// Danh sách package khoá theo `vendor/package` chữ thường.
    #[serde(default)]
    pub(in crate::protocols::composer) packages: BTreeMap<String, Vec<PackagistVersion>>,
}

/// One released version entry in a Packagist package list.
/// Một entry version đã phát hành trong danh sách package Packagist.
#[derive(Debug, Clone, Deserialize)]
pub(in crate::protocols::composer) struct PackagistVersion {
    #[serde(default)]
    pub(in crate::protocols::composer) version: String,
    #[serde(default, deserialize_with = "optional_dist")]
    pub(in crate::protocols::composer) dist: Option<PackagistDist>,
    /// Runtime dependencies (`require`).
    /// Phụ thuộc runtime (`require`).
    #[serde(default, deserialize_with = "dependency_map")]
    pub(in crate::protocols::composer) require: BTreeMap<String, String>,
    /// Development-only dependencies (`require-dev`).
    /// Phụ thuộc chỉ dùng khi phát triển (`require-dev`).
    #[serde(default, rename = "require-dev", deserialize_with = "dependency_map")]
    pub(in crate::protocols::composer) require_dev: BTreeMap<String, String>,
}

/// Distribution archive for one version.
/// Archive phân phối của một version.
#[derive(Debug, Clone, Deserialize)]
pub(in crate::protocols::composer) struct PackagistDist {
    #[serde(default)]
    pub(in crate::protocols::composer) url: String,
    #[serde(default, rename = "type")]
    pub(in crate::protocols::composer) archive_type: String,
    /// sha1 hex of the archive; Packagist commonly leaves it empty.
    /// sha1 hex của archive; Packagist thường để rỗng.
    #[serde(default)]
    pub(in crate::protocols::composer) shasum: Option<String>,
    /// VCS commit reference the archive was built from.
    /// Commit reference VCS mà archive được dựng từ đó.
    #[serde(default)]
    pub(in crate::protocols::composer) reference: Option<String>,
}
