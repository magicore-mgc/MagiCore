//! Verify invalid registry names are rejected before making a request.
//! Xác nhận tên registry không hợp lệ bị từ chối trước khi gửi request.

use super::super::PubProtocol;
use mgc_types::MgError;

#[tokio::test]
async fn package_doc_rejects_invalid_name_before_http() {
    let protocol = PubProtocol::new("http://127.0.0.1:1");
    let error = protocol
        .package_doc("../escape")
        .await
        .expect_err("invalid package names must fail before transport");

    assert!(matches!(error, MgError::InvalidPackageName(_)));
}
