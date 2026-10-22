//! Download, verify, CAS-import and materialize one Swift registry archive.
//! Tải, verify, import CAS và materialize một archive registry Swift.

use mgc_resolver::protocols::{RegistryProtocol, ResolvedEntry, SwiftRegistryProtocol};
use mgc_store::ContentStore;
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Preserve the verified archive lifecycle order used by the App Swift installer.
/// Giữ đúng thứ tự lifecycle archive đã verify trong installer Swift của App.
pub(in crate::install) async fn install_registry_archive(
    protocol: &SwiftRegistryProtocol,
    entry: &ResolvedEntry,
    store: &ContentStore,
    swift_root: &Path,
) -> MgResult<()> {
    let bytes = protocol.download(entry).await?;
    protocol.verify(entry, &bytes)?;
    store
        .import_bytes(&bytes)
        .map_err(|error| MgError::Store(error.to_string()))?;
    protocol.materialize(entry, &bytes, swift_root)?;
    Ok(())
}
