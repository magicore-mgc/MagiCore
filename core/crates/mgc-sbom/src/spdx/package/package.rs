//! SPDX package DTO — DTO package SPDX

use super::super::{SpdxChecksum, SpdxExternalRef};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::spdx) struct SpdxPackage {
    #[serde(rename = "SPDXID")]
    pub(in crate::spdx) spdx_id: String,
    pub(in crate::spdx) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::spdx) version_info: Option<String>,
    pub(in crate::spdx) download_location: &'static str,
    pub(in crate::spdx) files_analyzed: bool,
    pub(in crate::spdx) license_concluded: &'static str,
    pub(in crate::spdx) license_declared: &'static str,
    pub(in crate::spdx) copyright_text: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(in crate::spdx) checksums: Vec<SpdxChecksum>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(in crate::spdx) external_refs: Vec<SpdxExternalRef>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
