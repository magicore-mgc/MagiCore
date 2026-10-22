//! Missing model paths return an error.
//! Đường dẫn model không tồn tại phải trả lỗi.

use super::*;

#[tokio::test]
async fn test_audit_model_missing() {
    let result = audit_model(Path::new("/nonexistent")).await;
    assert!(result.is_err());
}
