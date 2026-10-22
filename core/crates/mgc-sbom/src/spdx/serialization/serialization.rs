//! SPDX document assembly and JSON output — Lắp tài liệu SPDX và xuất JSON

use crate::Bom;

use super::super::{
    CreationInfo, SpdxDocument,
    document_name_validation::validate_document_name,
    error::{SpdxError, SpdxResult},
    package_identity::build_package_ids,
    package_projection::project_packages,
    relationship_projection::project_relationships,
};

const SPDX_VERSION: &str = "SPDX-2.3";

pub(in crate::spdx) fn serialize(bom: &Bom, document_name: &str) -> SpdxResult<String> {
    validate_document_name(document_name)?;
    let package_ids = build_package_ids(bom)?;
    let packages = project_packages(bom, &package_ids)?;
    let (document_describes, relationships) = project_relationships(bom, &package_ids, &packages)?;

    let version = env!("CARGO_PKG_VERSION");
    let document = SpdxDocument {
        spdx_version: SPDX_VERSION,
        data_license: "CC0-1.0",
        spdx_id: "SPDXRef-DOCUMENT",
        name: document_name.to_string(),
        document_namespace: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
        creation_info: CreationInfo {
            creators: vec![format!("Tool: mgc-{version}")],
            created: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        },
        document_describes,
        packages,
        relationships,
    };
    serde_json::to_string_pretty(&document).map_err(SpdxError::Serialization)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
