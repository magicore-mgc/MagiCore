//! Tests safe URL-derived destination filenames.
//! Kiểm tra tên destination an toàn lấy từ URL.

use crate::install::download::filename_from_url;

#[test]
fn test_filename_from_url() {
    assert_eq!(filename_from_url("https://x.y/a/model.gguf"), "model.gguf");
    assert_eq!(filename_from_url("https://x.y/a/m.bin?token=1"), "m.bin");
    assert_eq!(
        filename_from_url("https://x.y/a/m.safetensors#frag"),
        "m.safetensors"
    );
    assert_eq!(
        filename_from_url("https://x.y/"),
        "model.bin",
        "rỗng → fallback mặc định"
    );
}
