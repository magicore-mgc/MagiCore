//! Existing ETag-persistence behavior test.
//! Test hành vi lưu ETag hiện có.

use super::*;

#[tokio::test]
async fn test_load_metadata_persists_etag_after_initial_fetch() {
    let shared = tempdir_real().unwrap();
    let Some(listener) = bind_test_listener().await else {
        return;
    };
    let addr = listener.local_addr().unwrap();
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_for_server = hits.clone();

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            hits_for_server.fetch_add(1, Ordering::SeqCst);
            let mut buf = [0u8; 2048];
            let _ = stream.read(&mut buf).await;
            let body = r#"{"name":"react","description":null,"versions":{"18.2.0":{"version":"18.2.0","dependencies":null,"optionalDependencies":null,"os":null,"cpu":null,"dist":{"tarball":"http://example.test/react.tgz","integrity":"sha512-react"}}},"dist-tags":{"latest":"18.2.0"}}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nETag: \"react-v1\"\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes()).await;
        }
    });

    let registry = native::npm_registry::NpmRegistry::new(&format!("http://{addr}"));
    let cache = SharedWebCache {
        root: shared.path().to_path_buf(),
    };

    let metadata = load_metadata_by_name_with_fallback("react", &registry, Some(&cache))
        .await
        .unwrap();
    assert_eq!(metadata.name, "react");
    assert_eq!(hits.load(Ordering::SeqCst), 1);

    let cached = std::fs::read_to_string(
        shared
            .path()
            .join("metadata")
            .join("v2")
            .join(reg_key(&format!("http://{addr}")))
            .join("react")
            .join("metadata.json"),
    )
    .unwrap();
    assert!(cached.contains("\"etag\":\"\\\"react-v1\\\"\""));
}
