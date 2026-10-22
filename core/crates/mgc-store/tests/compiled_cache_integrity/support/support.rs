//! Fixtures and deterministic helpers local to this integration-test binary.
//! Fixture và helper tất định, chỉ thuộc binary integration test này.

use mgc_store::cas::{CompilationKey, CompiledModule};
use std::fs;
use std::path::PathBuf;

pub(crate) const SRC: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

pub(crate) fn tmp_store_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .canonicalize()
        .unwrap_or_else(|_| std::env::temp_dir().to_path_buf())
        .join("magicore-compiled-cache")
        .join(format!(
            "{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

pub(crate) fn module(js: &str) -> CompiledModule {
    CompiledModule {
        js: js.to_string(),
        source_map: None,
    }
}

/// On-disk entry path, computed exactly like CompiledCache::module_path
/// (integration tests reach the store root through tmp_store_dir).
/// (Path entry trên đĩa, tính đúng như CompiledCache::module_path —
/// integration test tới được gốc store qua tmp_store_dir.)
pub(crate) fn entry_path(root: &std::path::Path, key: &CompilationKey) -> PathBuf {
    let master = key.master_digest();
    let hex = master.as_hex();
    root.join("compiled")
        .join("blake3")
        .join(&hex[..2])
        .join(format!("{hex}.json"))
}

/// BLAKE3 over js + NUL + source_map — mirrors the cache's output digest.
/// (BLAKE3 trên js + NUL + source_map — phản chiếu digest output của cache.)
pub(crate) fn out_digest(js: &str, sm: Option<&str>) -> String {
    // Mirrors the cache's schema-v2 length-prefixed encoding (P1-1).
    // (Phản chiếu mã hóa length-prefixed schema v2 của cache (P1-1).)
    let mut h = blake3::Hasher::new();
    h.update(b"MGC-CC-OV2");
    h.update(&(js.len() as u64).to_le_bytes());
    h.update(js.as_bytes());
    match sm {
        None => {
            h.update(&[0x00]);
        }
        Some(m) => {
            h.update(&[0x01]);
            h.update(&(m.len() as u64).to_le_bytes());
            h.update(m.as_bytes());
        }
    }
    h.finalize().to_hex().to_string()
}

/// Recursively scan for atomic-write temp leftovers under a store root.
/// (Quét đệ quy temp của ghi nguyên tử dưới gốc store.)
pub(crate) fn scan_temps(root: &std::path::Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("compiled-") {
                    found.push(path);
                }
            }
        }
    }
    found
}
