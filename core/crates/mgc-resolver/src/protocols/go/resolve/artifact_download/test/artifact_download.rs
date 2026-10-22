//! Verify that resolved artifact bytes come from the configured URL.
//! Kiểm tra bytes artifact được tải từ URL đã cấu hình.

use super::super::*;

#[tokio::test]
async fn downloads_bytes_from_the_resolved_artifact_url() {
    let mut server = mockito::Server::new_async().await;
    let mut entry = ResolvedEntry {
        name: "example.test/pkg".to_string(),
        version: "1.2.3".to_string(),
        deps: Vec::new(),
        artifact_url: String::new(),
        sha256: String::new(),
        extra_markers: Vec::new(),
    };
    entry.artifact_url = format!("{}/example.test/pkg/@v/v1.2.3.zip", server.url());
    server
        .mock("GET", "/example.test/pkg/@v/v1.2.3.zip")
        .with_status(200)
        .with_body("zip-payload")
        .create_async()
        .await;
    let protocol = GoModProtocol::with_sum_base(&server.url(), &server.url());

    assert_eq!(
        protocol
            .download_artifact(&entry)
            .await
            .expect("test operation must succeed"),
        b"zip-payload"
    );
}
