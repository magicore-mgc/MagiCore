//! SPDX document DTO — DTO tài liệu SPDX

use super::super::CreationInfo;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::spdx) struct SpdxDocument {
    pub(in crate::spdx) spdx_version: &'static str,
    pub(in crate::spdx) data_license: &'static str,
    #[serde(rename = "SPDXID")]
    pub(in crate::spdx) spdx_id: &'static str,
    pub(in crate::spdx) name: String,
    pub(in crate::spdx) document_namespace: String,
    pub(in crate::spdx) creation_info: CreationInfo,
    pub(in crate::spdx) document_describes: Vec<String>,
    pub(in crate::spdx) packages: Vec<super::super::SpdxPackage>,
    pub(in crate::spdx) relationships: Vec<super::super::SpdxRelationship>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
