//! Compiled module cache lookup and raw-output persistence.
//! Tra compiled module cache và lưu output raw.

use std::path::{Path, PathBuf};

pub(in crate::bundler::dev_server::implementation) fn cached_raw_js(
    store_root: &Path,
    source_hash: &str,
    loader: mgc_store::cas::Loader,
    options_json: &str,
    file_path: &Path,
) -> Option<String> {
    let Ok(store) = mgc_store::cas::ContentStore::new(store_root.to_path_buf()) else {
        return None;
    };
    let compiled_cache = store.compiled_cache();
    let Ok(comp_key) = mgc_store::cas::CompilationKey::new(
        source_hash,
        loader,
        super::super::compile_options::COMPILER_IDENTITY,
        super::super::compile_options::COMPILER_VERSION,
        options_json,
    ) else {
        return None;
    };
    match compiled_cache.get(&comp_key) {
        Ok(Some(cached)) => {
            tracing::debug!(
                "compiled-cache hit: {} ({}:{})",
                file_path.display(),
                loader,
                &source_hash[..8]
            );
            Some(cached.js)
        }
        Err(e) => {
            tracing::debug!(
                "compiled-cache entry rejected (quarantined), recompiling {}: {}",
                file_path.display(),
                e
            );
            None
        }
        Ok(None) => None,
    }
}

pub(in crate::bundler::dev_server::implementation) fn save_raw_js(
    store_root: PathBuf,
    source_hash: &str,
    loader: mgc_store::cas::Loader,
    options_json: &str,
    raw_js: String,
) {
    if let Ok(store) = mgc_store::cas::ContentStore::new(store_root) {
        let compiled_cache = store.compiled_cache();
        if let Ok(comp_key) = mgc_store::cas::CompilationKey::new(
            source_hash,
            loader,
            super::super::compile_options::COMPILER_IDENTITY,
            super::super::compile_options::COMPILER_VERSION,
            options_json,
        ) {
            let module = mgc_store::cas::CompiledModule {
                js: raw_js,
                source_map: None,
            };
            if let Err(e) = compiled_cache.put(&comp_key, &module) {
                tracing::debug!("failed to save to compiled cache: {}", e);
            }
        }
    }
}
