//! SPDX serializer facade — Facade bộ tuần tự hóa SPDX

mod checksum;
mod checksum_projection;
mod creation_info;
mod document;
mod document_name_validation;
mod error;
mod external_ref;
mod package;
mod package_identity;
mod package_projection;
mod relationship;
mod relationship_projection;
mod serialization;

use crate::{Bom, SbomResult};
use checksum::SpdxChecksum;
use creation_info::CreationInfo;
use document::SpdxDocument;
use error::SpdxError;
use external_ref::SpdxExternalRef;
use package::SpdxPackage;
use relationship::SpdxRelationship;

/// Convert a CycloneDX graph to SPDX JSON while retaining the public error boundary.
/// Chuyển graph CycloneDX sang SPDX JSON và giữ nguyên ranh giới lỗi public.
pub(crate) fn serialize(bom: &Bom, document_name: &str) -> SbomResult<String> {
    serialization::serialize(bom, document_name).map_err(SpdxError::into_public)
}
