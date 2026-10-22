//! Temporary Lib project fixture — fixture project Lib tạm.
//! Shared across Lib integration behavior owners. — Dùng chung cho các owner integration Lib.

use std::path::PathBuf;

pub(crate) fn tmp(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-lib-itg-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("tmp dir");
    dir
}
