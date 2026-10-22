//! Tests bounded streaming and rejection before an oversized chunk is written.
//! Kiểm tra streaming có giới hạn và từ chối trước khi ghi chunk quá cỡ.

use super::support::{serve_chunked_once, tmp};
use crate::install::download::http_download_to_staging_with_limit;

#[tokio::test]
async fn remote_download_quota_rejects_before_writing_over_limit_chunk() {
    let tmp = tmp();
    let staging = tmp.path().join("bounded.part");
    let (url, server) = serve_chunked_once(b"12345");

    let error = http_download_to_staging_with_limit(&url, &staging, 4)
        .await
        .expect_err("response larger than the configured limit must be rejected");
    server.join().unwrap();
    assert!(error.to_string().contains("size limit"), "{error}");
    assert_eq!(
        std::fs::metadata(&staging).unwrap().len(),
        0,
        "the over-limit response chunk must be rejected before it is written"
    );
}
