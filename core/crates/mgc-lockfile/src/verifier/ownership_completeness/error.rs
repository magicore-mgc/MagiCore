//! Typed provenance-completeness failure mapped to the stable crate error.
//! Lỗi thiếu provenance có kiểu, ánh xạ về lỗi crate ổn định.

/// A classified package lacks a complete source identity.
/// Package đã phân loại thiếu định danh nguồn đầy đủ.
#[derive(Debug, thiserror::Error)]
pub(super) enum OwnershipCompletenessError {
    #[error(
        "package '{name}@{version}' (ecosystem {ecosystem}) has no complete source identity (registry, imported lockfile, or named delegated tool)"
    )]
    MissingSource {
        name: String,
        version: String,
        ecosystem: crate::EcosystemTag,
    },
}

impl From<OwnershipCompletenessError> for crate::LockfileError {
    fn from(error: OwnershipCompletenessError) -> Self {
        Self::IncompleteProvenance(error.to_string())
    }
}
