// Temporary IoT project fixture helper.
// Helper tạo fixture dự án IoT tạm dùng chung.
use std::path::PathBuf;

pub fn tmp(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-iot-itg-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("tmp dir");
    dir
}
