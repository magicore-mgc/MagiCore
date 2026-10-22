// cas_pull owner — cas_pull owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::super::huggingface::pull_hf;
use super::super::manifest_io::{now_iso, save_manifest};
use super::super::manifest_path::{model_manifest_path, store_root};
use super::super::manifest_types::ModelManifest;
use super::super::oci_pull::pull_oci;
use super::super::types::ModelOptionsError;
use super::CasPullError;
use anyhow::Result;
use std::path::Path;

const DEFAULT_MODEL_PULL_MAX_BYTES: u64 = 64 * 1024 * 1024 * 1024;
/// Import an external HF/OCI source into the CAS and commit its manifest.
/// CAS pull — tải nguồn ngoài (HF/OCI) vào CAS store + manifest.
pub(in crate::commands::model) async fn cas_pull(
    source: &str,
    revision: Option<&str>,
    sha256: Option<&str>,
    max_bytes: Option<u64>,
) -> Result<()> {
    let store = mgc_store::cas::ContentStore::new(store_root())?;
    let (name, blobs, total, recorded_source) = if let Some(hf) = source.strip_prefix("hf://") {
        let max_bytes = max_bytes.unwrap_or(DEFAULT_MODEL_PULL_MAX_BYTES);
        if max_bytes == 0 {
            return Err(CasPullError::MaximumBytesMustBePositive.into());
        }
        let (name, blobs, total, pinned_source) =
            pull_hf(&store, hf, revision, sha256, max_bytes).await?;
        (name, blobs, total, pinned_source)
    } else if let Some(oci) = source.strip_prefix("oci://") {
        if revision.is_some() || sha256.is_some() || max_bytes.is_some() {
            return Err(ModelOptionsError::PullFlagsRequireHf.into());
        }
        let (name, blobs, total) = pull_oci(&store, oci).await?;
        (name, blobs, total, source.to_string())
    } else {
        return Err(CasPullError::UnsupportedSource(source.to_string()).into());
    };

    let manifest = ModelManifest {
        name,
        source: recorded_source,
        total_bytes: total,
        pulled_at: now_iso(),
        blobs,
    };
    save_manifest(&manifest)?;
    println!(
        "pulled {} → CAS ({} bytes, manifest at {})",
        manifest.name,
        manifest.total_bytes,
        model_manifest_path(&manifest.name)?.display()
    );
    Ok(())
}

pub(in crate::commands::model) fn cas_import(
    store: &mgc_store::cas::ContentStore,
    src: &Path,
) -> Result<(String, u64)> {
    let len = std::fs::metadata(src)?.len();
    let hash = store.import_file(src)?;
    // Accessor (P0-1): IntegrityHash fields are private — read via as_hex().
    // (Accessor (P0-1): field IntegrityHash đã private — đọc qua as_hex().)
    Ok((hash.as_hex().to_string(), len))
}
