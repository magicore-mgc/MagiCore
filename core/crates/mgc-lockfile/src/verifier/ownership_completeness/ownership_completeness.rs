//! Validates registry or delegated-tool provenance independently of signatures.
//! Xác thực registry hoặc provenance delegated-tool độc lập chữ ký.

use crate::schema::{SOURCE_KIND_DELEGATED_TOOL, SOURCE_KIND_REGISTRY_IMPORT};
use crate::{EcosystemTag, Lockfile, LockfileResult};

use super::error::OwnershipCompletenessError;

/// Require a complete source identity for every classified package.
/// Bắt mọi package đã phân loại có định danh nguồn đầy đủ.
pub fn verify_ownership_completeness(lockfile: &Lockfile) -> LockfileResult<()> {
    for package in &lockfile.packages {
        if package.ecosystem == EcosystemTag::Other {
            continue;
        }

        let has_registry = package
            .registry
            .as_deref()
            .is_some_and(|registry| !registry.trim().is_empty());
        let has_delegated_provenance = package.provenance.as_ref().is_some_and(|provenance| {
            provenance.source_kind == SOURCE_KIND_DELEGATED_TOOL
                && provenance
                    .tool
                    .as_deref()
                    .is_some_and(|tool| !tool.trim().is_empty())
        });

        let has_import_provenance = package.provenance.as_ref().is_some_and(|provenance| {
            provenance.source_kind == SOURCE_KIND_REGISTRY_IMPORT
                && provenance
                    .imported_from
                    .as_deref()
                    .is_some_and(|source| !source.trim().is_empty())
        });

        if !has_registry && !has_delegated_provenance && !has_import_provenance {
            return Err(OwnershipCompletenessError::MissingSource {
                name: package.name.clone(),
                version: package.version.clone(),
                ecosystem: package.ecosystem,
            }
            .into());
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
