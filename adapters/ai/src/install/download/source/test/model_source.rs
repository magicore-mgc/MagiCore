//! Tests public model-source construction.
//! Kiểm tra khởi tạo model source public.

use crate::install::download::ModelSource;

#[test]
fn test_model_source_builders() {
    let hf = ModelSource::huggingface("gpt2", "config.json");
    assert!(matches!(hf, ModelSource::HuggingFace { .. }));
    assert_eq!(hf.checksum(), None);

    let url = ModelSource::url("https://example.com/model.bin");
    assert!(matches!(url, ModelSource::Url { .. }));

    // with_checksum gắn được cho cả HF lẫn URL
    // (with_checksum attaches to both HF and Url variants)
    let hf_ck = ModelSource::huggingface("gpt2", "config.json").with_checksum("sha256:abc123");
    assert_eq!(hf_ck.checksum(), Some("sha256:abc123"));

    let url_ck = ModelSource::url("https://example.com/m.bin").with_checksum("blake3:deadbeef");
    assert_eq!(url_ck.checksum(), Some("blake3:deadbeef"));

    let local_ck =
        ModelSource::Local(std::path::PathBuf::from("/tmp/x.bin")).with_checksum("sha256:abc");
    assert_eq!(local_ck.checksum(), None, "Local không mang checksum");
}
