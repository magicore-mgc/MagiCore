//! Temporary project directory fixture — fixture thư mục project tạm.
//! Shared by AI adapter integration behavior owners. — Dùng chung cho các owner integration AI.

use std::path::PathBuf;

pub(crate) fn tmp(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-ai-itg-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("tmp dir");
    dir
}
