//! Construct the shared MGC content store for native App install lanes.
//! Tạo content store MGC dùng chung cho lane install App native.
use mgc_store::ContentStore;
use mgc_types::{MgError, MgResult};

/// Build (and create) the mgc CAS content store used by native install.
/// Dựng (và tạo) content store CAS mgc cho install native.
pub(crate) fn content_store() -> MgResult<ContentStore> {
    ContentStore::new(mgc_store::default_store_root()).map_err(|e| MgError::Store(e.to_string()))
}
