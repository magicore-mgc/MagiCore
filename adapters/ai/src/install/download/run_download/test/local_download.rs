//! Tests local-source copy through the download orchestration API.
//! Kiểm tra copy nguồn local qua API điều phối download.

use super::support::{block_on, tmp};
use crate::install::download::{ModelSource, download_model};

#[test]
fn test_download_local_copies_content() {
    let tmp = tmp();
    let src = tmp.path().join("model.onnx");
    std::fs::write(&src, b"fake onnx").unwrap();

    let target = tmp.path().join("target");
    let source = ModelSource::Local(src);

    let (path, bytes) = block_on(download_model("test", &source, &target));
    assert!(path.exists());
    assert_eq!(bytes, 9);
    assert_eq!(
        std::fs::read(&path).unwrap(),
        b"fake onnx".to_vec(),
        "local copy must preserve content"
    );
}
