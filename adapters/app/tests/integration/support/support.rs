//! Temporary App project fixture — fixture project App tạm.
//! Shared across App integration behavior owners. — Dùng chung giữa các owner integration App.

use std::path::PathBuf;

pub(crate) fn tmp(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-app-itg-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("tmp dir");
    dir
}
