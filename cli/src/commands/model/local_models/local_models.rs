// local_models owner — local_models owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::super::manifest_io::read_manifests_in;
use super::super::manifest_path::{
    ManifestPathError, model_manifest_dir, model_manifest_path, store_root,
};
use super::super::manifest_types::ModelManifest;
use super::LocalModelError;
use anyhow::{Context, Result};
/// List local models from their CAS manifests.
/// Liệt kê model local (CAS manifest).
pub(in crate::commands::model) fn list_local() -> Result<()> {
    let manifests = read_manifests_in(model_manifest_dir());
    if manifests.is_empty() {
        println!("(no local models — pull one: `mgc model pull hf://org/model/file`)");
        return Ok(());
    }
    for m in manifests {
        println!(
            "{}
  source: {}
  {} bytes, {} blob(s), pulled {}",
            m.name,
            m.source,
            m.total_bytes,
            m.blobs.len(),
            m.pulled_at
        );
    }
    Ok(())
}

/// Remove a local model and delete only blobs with no remaining manifest references.
/// Xoá model local — manifest + blob CAS chỉ khi không còn manifest nào trỏ (refcount).
pub(in crate::commands::model) fn remove_local(name: &str) -> Result<()> {
    let path = model_manifest_path(name)?;
    match std::fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_file() => {}
        Ok(_) => return Err(ManifestPathError::UnsafeManifestPath.into()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(LocalModelError::NotFound {
                name: name.to_string(),
                manifest: path,
            }
            .into());
        }
        Err(error) => return Err(error.into()),
    }
    let manifest: ModelManifest = serde_json::from_str(&std::fs::read_to_string(&path)?)
        .with_context(crate::error::parse_manifest_failed)?;

    let all: Vec<ModelManifest> = read_manifests_in(model_manifest_dir());
    let others: Vec<&str> = all
        .iter()
        .filter(|m| m.name != name)
        .flat_map(|m| m.blobs.iter().map(|b| b.as_str()))
        .collect();

    let store = mgc_store::cas::ContentStore::new(store_root())?;
    for blob in &manifest.blobs {
        if others.contains(&blob.as_str()) {
            continue; // còn model khác dùng — giữ blob (refcount T1)
        }
        // Manifest hash is external input — fail-closed validation, then warn
        // and skip the blob (deletion must not abort the whole removal).
        // Hash trong manifest là input ngoài — validate fail-closed, nếu sai
        // dạng thì cảnh báo và bỏ qua blob (không abort cả lệnh remove).
        let hash = match mgc_store::cas::IntegrityHash::from_hash_str(blob, false) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("warning: invalid blob hash '{blob}', skipping: {e}");
                continue;
            }
        };
        if let Err(e) = store.remove(&hash) {
            eprintln!("warning: failed to remove blob {blob}: {e}");
        }
    }
    std::fs::remove_file(&path)?;
    println!("removed model '{name}'");
    Ok(())
}
