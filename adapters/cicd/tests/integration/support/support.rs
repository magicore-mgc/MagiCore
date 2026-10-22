// Temporary project fixture directory helper.
// Hàm tạo thư mục dự án tạm dùng chung cho integration tests.
use std::path::PathBuf;

pub fn tmp(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-cicd-itg-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("tmp dir");
    dir
}
