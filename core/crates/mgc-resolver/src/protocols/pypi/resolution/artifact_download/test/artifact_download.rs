//! Successful response bytes pass through unchanged.
//! Byte response thành công được giữ nguyên.

use super::super::download;
use crate::protocols::ResolvedEntry;

#[tokio::test]
async fn returns_the_successful_artifact_body() {
    let mut server = mockito::Server::new_async().await;
    let body = b"pypi-wheel-bytes";
    let mock = server
        .mock("GET", "/files/demo.whl")
        .with_status(200)
        .with_body(body.as_slice())
        .create_async()
        .await;
    let entry = ResolvedEntry {
        name: "demo".to_string(),
        version: "1.0.0".to_string(),
        deps: Vec::new(),
        artifact_url: format!("{}/files/demo.whl", server.url()),
        sha256: String::new(),
        extra_markers: Vec::new(),
    };
    let client = mgc_http::HttpClient::default();

    let downloaded = download(&client, &entry)
        .await
        .expect("test operation must succeed");

    assert_eq!(downloaded.as_slice(), b"pypi-wheel-bytes".as_slice());
    mock.assert_async().await;
}
