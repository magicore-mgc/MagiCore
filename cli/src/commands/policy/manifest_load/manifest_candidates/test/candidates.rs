//! Keep explicit, project and shared manifest paths ordered.
//! Giữ thứ tự đường dẫn manifest explicit, project và shared.

#![allow(clippy::unwrap_used)]

use super::candidate_paths;
use std::path::{Path, PathBuf};

#[test]
fn explicit_project_and_shared_paths_keep_precedence() {
    let root = PathBuf::from("project-root");
    let explicit = PathBuf::from("user-policy.json");
    let shared = Path::new("shared-data");
    let candidates = candidate_paths(&root, Some(explicit.clone()), shared);

    assert_eq!(candidates[0], explicit);
    assert_eq!(
        candidates[1],
        root.join(".magicore")
            .join(mgc_exec::policy_manifest::cache_rel_path())
    );
    assert_eq!(
        candidates[2],
        shared.join(mgc_exec::policy_manifest::cache_rel_path())
    );
}
