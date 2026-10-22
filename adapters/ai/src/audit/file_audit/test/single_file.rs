//! Single model files use the audit dispatcher.
//! File model đơn được đưa qua bộ điều phối audit.

use super::*;

#[tokio::test]
async fn test_audit_single_file() {
    let tmp = tmp();
    let model = tmp.path().join("model.safetensors");
    std::fs::write(&model, b"fake safetensors").unwrap();

    let report = audit_model(&model).await.unwrap();
    assert_eq!(report.scanned_files, 1);
}
