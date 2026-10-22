//! Discover policy manifest files in the established precedence order.
//! Tìm file manifest policy theo thứ tự ưu tiên hiện hành.

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::commands::policy::manifest_cache::MAX_POLICY_MANIFEST_BYTES;

// The executor consults manifests only with an active lock-bound profile.
// Missing or invalid inputs keep native-deny in force.
// Executor chỉ tra manifest cùng profile hiệu lực gắn lock. Thiếu hoặc sai
// input thì native-deny tiếp tục được áp dụng.
/// Environment override pointing at one manifest file.
/// Env override chỉ tới một file manifest.
#[allow(dead_code)]
pub const POLICY_MANIFEST_ENV: &str = "MGC_POLICY_MANIFEST";

fn candidate_paths(
    project_root: &Path,
    explicit_path: Option<PathBuf>,
    shared_data_dir: &Path,
) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    if let Some(path) = explicit_path {
        paths.push(path);
    }
    paths.push(
        project_root
            .join(".magicore")
            .join(mgc_exec::policy_manifest::cache_rel_path()),
    );
    paths.push(shared_data_dir.join(mgc_exec::policy_manifest::cache_rel_path()));
    paths
}

/// Collect candidate manifest documents without failing the caller on
/// unreadable files: a corrupt optional policy file warns and is skipped —
/// the base native-deny layers stay enforced regardless.
/// (Gom tài liệu manifest ứng viên mà không fail caller khi file lỗi đọc:
/// file policy tùy nghi hỏng thì cảnh báo và bỏ qua — các lớp native-deny
/// gốc vẫn thực thi dù thế nào.)
#[allow(dead_code)]
pub(in crate::commands::policy::manifest_load) fn read_candidates(
    project_root: &Path,
) -> Vec<mgc_exec::policy_manifest::PolicyManifest> {
    let explicit_path = std::env::var_os(POLICY_MANIFEST_ENV)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    let shared_data_dir = mgc_platform::paths::mgc_data_dir();
    let paths = candidate_paths(project_root, explicit_path, &shared_data_dir);

    let mut manifests = Vec::new();
    for path in paths {
        let mut bytes = Vec::new();
        let read_result = std::fs::File::open(&path).and_then(|file| {
            file.take((MAX_POLICY_MANIFEST_BYTES + 1) as u64)
                .read_to_end(&mut bytes)
        });
        match read_result {
            Ok(size) if size <= MAX_POLICY_MANIFEST_BYTES => {}
            Ok(_) => {
                mgc_ui::warning(&format!(
                    "ignoring oversized policy manifest '{}': maximum size is 1 MiB",
                    path.display()
                ));
                continue;
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => {
                mgc_ui::warning(&format!(
                    "ignoring unreadable policy manifest '{}': {e}",
                    path.display()
                ));
                continue;
            }
        }
        match serde_json::from_slice(&bytes) {
            Ok(manifest) => manifests.push(manifest),
            Err(e) => {
                mgc_ui::warning(&format!(
                    "ignoring malformed policy manifest '{}': {e}",
                    path.display()
                ));
            }
        }
    }
    manifests
}

#[cfg(test)]
#[path = "test/candidates.rs"]
mod tests;
