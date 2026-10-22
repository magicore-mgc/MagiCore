//! `v4_short_circuit.rs` — v4 lockfile short-circuit / tái dùng lockfile v4.
use mgc_types::{Manifest, MgError, MgResult, adapter::ResolvedGraph};

/// v4 resolve short-circuit: reuse a satisfying v4 lock graph instead of
/// re-solving. Returns `None` (miss) for missing/non-v4 locks, closure
/// mismatches, and lossy-but-valid v4 shapes — all fall through to fresh
/// resolve exactly like a legacy short-circuit miss. Corrupt (version-4
/// but unparseable) files fail like the legacy reader; the CLI authority
/// path separately enforces hardness for lossy v4 inside install.
/// (Short-circuit v4: lock v4 thỏa thì tái dùng, còn lại miss về resolve
/// mới như legacy; file v4 hỏng thì lỗi như reader cũ.)
pub(in crate::adapter::resolution) fn try_v4_short_circuit(
    manifest: &Manifest,
) -> MgResult<Option<ResolvedGraph>> {
    use std::io::ErrorKind;
    let lock_path = std::path::Path::new(".").join("mgc.lock");
    let bytes = match mgc_lockfile::read_lockfile_bytes(&lock_path) {
        Ok(bytes) => bytes,
        Err(mgc_lockfile::LockfileError::IoError(error)) if error.kind() == ErrorKind::NotFound => {
            return Ok(None);
        }
        Err(error) => {
            return Err(MgError::Other(format!("Failed to read lockfile: {error}")));
        }
    };
    let content = String::from_utf8(bytes)
        .map_err(|error| MgError::Other(format!("Lockfile is not valid UTF-8: {error}")))?;
    if !mgc_lockfile::detect_lockfile_version(&content).is_ok_and(|version| version == 4) {
        return Ok(None);
    }
    let doc = mgc_lockfile::canonical::parse_v4_document(&content)
        .map_err(|error| MgError::Other(format!("Failed to parse lockfile: {error}")))?;
    let verification = crate::lockfile::verify_v4_document_integrity(&doc)?;
    if !verification.signed {
        mgc_ui::warning(
            "mgc.lock v4 digest is valid, but the lock is unsigned; signer trust was not evaluated",
        );
    }
    let owner = crate::lockfile::web_lock_owner_core(std::path::Path::new("."))?;
    if !mgc_lockfile::v4_graph::v4_graph_matches_manifest_closure(&doc, manifest, &owner) {
        return Ok(None);
    }
    match mgc_lockfile::v4_graph::graph_from_v4_lockfile(&doc, &owner) {
        Ok(graph) => Ok(Some(graph)),
        Err(_) => Ok(None),
    }
}
