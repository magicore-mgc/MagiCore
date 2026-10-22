//! Alias-aware metadata request deduplication / gộp request metadata qua alias.

use crate::provider::NpmDependencyProvider;
use mgc_types::PackageName;
use std::io::ErrorKind;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn bind_test_listener() -> Option<TcpListener> {
    match TcpListener::bind("127.0.0.1:0").await {
        Ok(listener) => Some(listener),
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            eprintln!("skipping socket-backed test in sandbox: {err}");
            None
        }
        Err(err) => panic!("failed to bind socket-backed test listener: {err}"),
    }
}

#[tokio::test]
async fn test_prefetch_resolution_metadata_dedupes_aliases_by_source_package() {
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
            let body = concat!(
                r#"{"name":"strip-ansi","description":null,"versions":{"6.0.1":{"version":"6.0.1",""#,
                r#"dependencies":null,"optionalDependencies":null,"os":null,"cpu":null,"dist":{"tarball":""#,
                r#"http://example.test/strip-ansi.tgz","integrity":"sha512-strip"}}},"dist-tags":{"latest":"6.0.1"}}"#
            );
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes()).await;
        }
    });

    let provider = NpmDependencyProvider::new(&format!("http://{addr}"), None, None);
    let alias_a = PackageName::new("strip-ansi-a").unwrap();
    let alias_b = PackageName::new("strip-ansi-b").unwrap();
    let source = PackageName::new("strip-ansi").unwrap();
    provider.record_alias_target(&alias_a, &source);
    provider.record_alias_target(&alias_b, &source);

    let metadata = provider
        .prefetch_resolution_metadata(&[alias_a.clone(), alias_b.clone()])
        .await
        .unwrap();

    assert_eq!(metadata.len(), 2);
    assert_eq!(metadata[alias_a.as_str()].name, "strip-ansi");
    assert_eq!(metadata[alias_b.as_str()].name, "strip-ansi");
    assert_eq!(hits.load(Ordering::SeqCst), 1);
}
