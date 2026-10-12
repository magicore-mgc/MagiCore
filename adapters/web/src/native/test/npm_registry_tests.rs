#![cfg(test)]
#![allow(clippy::unwrap_used)]

// NPM registry tests for core-web — kept beside the native module test tree.
// Test registry NPM của core-web — tách khỏi thân file production để dễ mở rộng.
use super::*;
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
async fn test_fetch_metadata_retries_after_transient_failure() {
    let hits = Arc::new(AtomicUsize::new(0));
    let Some(listener) = bind_test_listener().await else {
        return;
    };
    let addr = listener.local_addr().unwrap();
    let hits_for_server = hits.clone();

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let hit = hits_for_server.fetch_add(1, Ordering::SeqCst);
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf).await;
            if hit == 0 || hit == 1 {
                let _ = stream
                    .write_all(
                        b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 3\r\n\r\nbad",
                    )
                    .await;
            } else {
                let body = r#"{"name":"react","description":null,"versions":{"18.2.0":{"version":"18.2.0","dependencies":null,"optionalDependencies":null,"os":null,"cpu":null,"dist":{"tarball":"http://example.test/react.tgz","integrity":null}}},"dist-tags":{"latest":"18.2.0"}}"#;
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(response.as_bytes()).await;
            }
        }
    });

    let registry = NpmRegistry::new(&format!("http://{}", addr));
    let metadata = registry.fetch_metadata("react").await.unwrap();
    assert_eq!(metadata.name, "react");
    assert!(hits.load(Ordering::SeqCst) >= 3);
}

#[tokio::test]
async fn concurrent_stream_downloads_write_to_distinct_staging_files() {
    let Some(listener) = bind_test_listener().await else {
        return;
    };
    let address = listener.local_addr().unwrap();
    let payload = Arc::new(vec![b'z'; 2 * 1024 * 1024 + 1]);
    let server_payload = Arc::clone(&payload);
    let server = tokio::spawn(async move {
        let mut handlers = Vec::new();
        for _ in 0..2 {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let payload = Arc::clone(&server_payload);
            handlers.push(tokio::spawn(async move {
                let mut request = [0u8; 4096];
                let _ = stream.read(&mut request).await;
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    payload.len()
                );
                stream.write_all(header.as_bytes()).await.unwrap();
                stream.write_all(&payload).await.unwrap();
            }));
        }
        for handler in handlers {
            handler.await.unwrap();
        }
    });

    let temp = tempfile::tempdir().unwrap();
    let first_final = temp.path().join("first.tgz");
    let second_final = temp.path().join("second.tgz");
    let first_staging = crate::install::fetch::unique_tarball_staging_path(&first_final);
    let second_staging = crate::install::fetch::unique_tarball_staging_path(&second_final);
    assert_ne!(first_staging, second_staging);

    let registry = NpmRegistry::new(&format!("http://{address}"));
    let url = format!("http://{address}/payload.tgz");
    let (first, second) = tokio::join!(
        registry.download_tarball_to_file(&url, &first_staging),
        registry.download_tarball_to_file(&url, &second_staging)
    );

    assert_eq!(first.unwrap(), second.unwrap());
    assert_eq!(std::fs::read(&first_staging).unwrap(), *payload);
    assert_eq!(std::fs::read(&second_staging).unwrap(), *payload);
    server.await.unwrap();
}

#[test]
fn test_check_publish_age_blocks_new_package() {
    let mut meta = PackageMetadata {
        name: "evil-pkg".to_string(),
        description: None,
        versions: Default::default(),
        dist_tags: Default::default(),
        time: Default::default(),
    };
    let published_at = (chrono::Utc::now() - chrono::Duration::hours(1)).to_rfc3339();
    meta.time.insert("1.0.0".to_string(), published_at);

    let result = check_publish_age(&meta, "1.0.0", 86400);
    assert!(result.is_err(), "should block packages published < 24h ago");
    assert!(result.unwrap_err().contains("quarantine"));

    let result = check_publish_age(&meta, "1.0.0", 1800);
    assert!(result.is_ok());
}

#[test]
fn test_check_publish_age_allows_old_package() {
    let mut meta = PackageMetadata {
        name: "safe-pkg".to_string(),
        description: None,
        versions: Default::default(),
        dist_tags: Default::default(),
        time: Default::default(),
    };
    let published_at = (chrono::Utc::now() - chrono::Duration::hours(48)).to_rfc3339();
    meta.time.insert("2.0.0".to_string(), published_at);

    let result = check_publish_age(&meta, "2.0.0", 86400);
    assert!(result.is_ok());
}

#[test]
fn test_base64_encode_hello() {
    assert_eq!(base64_encode(b"Hello"), "SGVsbG8=");
}

// Credential headers must respect the exact registry HTTP origin.
// Token không được gửi sang scheme, host hoặc port khác.
#[test]
fn registry_token_is_scoped_to_http_origin() {
    let registry = NpmRegistry::new_with_token(
        "https://registry.example:443/team/",
        Some("synthetic-fixture".to_string()),
    );
    let client = reqwest::Client::new();
    for (target, allowed) in [
        ("https://REGISTRY.example/team/pkg.tgz", true),
        ("https://registry.example:443/pkg.tgz", true),
        ("https://registry.example:4351/pkg.tgz", false),
        ("http://registry.example:443/pkg.tgz", false),
        ("https://foreign.example/pkg.tgz", false),
        ("https://fixture@registry.example/pkg.tgz", false),
    ] {
        let request = registry
            .with_auth(client.get(target), target)
            .build()
            .unwrap();
        assert_eq!(
            request
                .headers()
                .get(reqwest::header::AUTHORIZATION)
                .is_some_and(|value| value.as_bytes().starts_with(b"Bearer ")),
            allowed
        );
    }
    assert!(!registry_origin_matches(
        "invalid",
        "https://registry.example"
    ));
    assert!(!registry_origin_matches(
        "file:///fixture",
        "file:///fixture"
    ));
}

// A protocol-changing redirect must fail before any redirected connection.
// Redirect đổi scheme phải dừng trước khi kết nối tới đích mới.
#[tokio::test]
async fn registry_redirect_rejects_scheme_change() {
    const FIXTURE_PORTS: &[u16] = &[1534, 1543, 5134, 5143];
    const FIXTURE_ACCEPT_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(200);
    let mut selected = None;
    for port in FIXTURE_PORTS {
        if let Ok(listener) = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, *port)).await {
            selected = Some(listener);
            break;
        }
    }
    let listener = selected.expect("a permitted fixture port must be available");
    let port = listener.local_addr().unwrap().port();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut bytes = [0u8; 4096];
        let bytes_read = stream.read(&mut bytes).await.unwrap();
        assert!(
            bytes_read > 0,
            "redirect fixture requires a nonempty request"
        );
        let response = format!(
            "HTTP/1.1 302 Found\r\nLocation: https://127.0.0.1:{port}/final\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
        );
        stream.write_all(response.as_bytes()).await.unwrap();
        drop(stream);
        assert!(
            tokio::time::timeout(FIXTURE_ACCEPT_TIMEOUT, listener.accept())
                .await
                .is_err()
        );
    });
    let client = reqwest::Client::builder()
        .redirect(registry_redirect_policy())
        .build()
        .unwrap();
    let error = client
        .get(format!("http://127.0.0.1:{port}/initial"))
        .send()
        .await
        .unwrap_err();
    assert!(format!("{error:?}").contains("changes URL scheme"));
    server.await.unwrap();
}

// Verify typed credential authorization on actual request headers.
// Token theo scope đúng; request public và path traversal không nhận credential.
#[test]
fn scoped_request_headers_respect_path_boundary() {
    let mut row = mgc_config::registry::Registry::new(
        "fixture".into(),
        "https://registry.example/team/".into(),
    );
    row.token = Some("synthetic-fixture".into());
    let entry = mgc_config::chain::ScopedRegistry {
        registry: row,
        credential_scope: Some(reqwest::Url::parse("https://registry.example/team/").unwrap()),
    };
    for (path, allowed) in [
        ("/team/pkg", true),
        ("/team/@org%2fpkg", true),
        ("/foreign/pkg", false),
        ("/team-other/pkg", false),
        ("/team/%2f..%2fforeign", false),
        ("/team/%252f..%252fforeign", false),
    ] {
        let request = scoped_registry_request(&entry, &format!("https://registry.example{path}"))
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(
            request
                .headers()
                .contains_key(reqwest::header::AUTHORIZATION),
            allowed
        );
    }
}

// Network proof that scoped credentials cannot leave their path on redirects.
// Đối chứng trong scope có token; ngoài scope dừng trước request thứ hai.
#[tokio::test]
async fn scoped_request_redirect_network_boundary() {
    const PORTS: &[u16] = &[5314, 5341, 5413, 5431];
    const ACCEPT_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(250);
    for outside in [false, true] {
        let mut selected = None;
        for port in PORTS {
            if let Ok(listener) = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, *port)).await {
                selected = Some(listener);
                break;
            }
        }
        let listener = selected.expect("a permitted credential fixture port must be available");
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut bytes = [0u8; 4096];
            let read = stream.read(&mut bytes).await.unwrap();
            let initial = String::from_utf8_lossy(&bytes[..read]).to_ascii_lowercase();
            assert!(initial.contains("authorization: bearer synthetic-fixture"));
            let location = if outside {
                "/foreign/final"
            } else {
                "/team/final"
            };
            stream.write_all(format!("HTTP/1.1 302 Found\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").as_bytes()).await.unwrap();
            drop(stream);
            let next = tokio::time::timeout(ACCEPT_TIMEOUT, listener.accept()).await;
            if outside {
                assert!(next.is_err());
            } else {
                let (mut stream, _) = next.unwrap().unwrap();
                let read = stream.read(&mut bytes).await.unwrap();
                let final_request = String::from_utf8_lossy(&bytes[..read]).to_ascii_lowercase();
                assert!(final_request.contains("authorization: bearer synthetic-fixture"));
                stream
                    .write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK",
                    )
                    .await
                    .unwrap();
            }
        });
        let url = format!("http://127.0.0.1:{port}/team/");
        let mut row = mgc_config::registry::Registry::new("fixture".into(), url.clone());
        row.token = Some("synthetic-fixture".into());
        let entry = mgc_config::chain::ScopedRegistry {
            registry: row,
            credential_scope: Some(reqwest::Url::parse(&url).unwrap()),
        };
        let result = scoped_registry_request(&entry, &format!("{url}initial"))
            .unwrap()
            .send()
            .await;
        if outside {
            assert!(
                format!("{:?}", result.unwrap_err()).contains("leaves authorized registry scope")
            );
        } else {
            assert_eq!(result.unwrap().status(), reqwest::StatusCode::OK);
        }
        server.await.unwrap();
    }
}

// Refuse URL credentials even on a public no-token request.
// Userinfo không được tự sinh Basic auth ngầm ngoài scope token.
#[test]
fn scoped_request_rejects_url_userinfo() {
    let entry = mgc_config::chain::ScopedRegistry {
        registry: mgc_config::registry::Registry::new(
            "fixture".into(),
            "https://registry.example/team/".into(),
        ),
        credential_scope: None,
    };
    assert!(
        scoped_registry_request(
            &entry,
            "https://fixture-user:fixture-password@registry.example/public"
        )
        .is_err()
    );
}

// A single transport alternates scoped and public requests without header carryover.
// Tái dùng instance nhiều lượt; request public không được kế thừa token lượt trước.
#[test]
fn scoped_client_reuse_does_not_carry_credentials_to_public_requests() {
    let mut row = mgc_config::registry::Registry::new(
        "fixture".into(),
        "https://registry.example/team/".into(),
    );
    row.token = Some("synthetic-fixture".into());
    let entry = mgc_config::chain::ScopedRegistry {
        registry: row,
        credential_scope: Some(reqwest::Url::parse("https://registry.example/team/").unwrap()),
    };
    let transport = ScopedRegistryClient::new(&entry).unwrap();
    for _ in 0..32 {
        for (path, authorized) in [("/team/pkg", true), ("/foreign/pkg", false)] {
            let request = transport
                .get(&format!("https://registry.example{path}"))
                .unwrap()
                .build()
                .unwrap();
            assert_eq!(
                request
                    .headers()
                    .contains_key(reqwest::header::AUTHORIZATION),
                authorized
            );
        }
    }
}

// Invalid typed credentials must fail before any network request.
// Caller không thể cung cấp token rồi bỏ scope để transport suy đoán quyền.
#[test]
fn scoped_client_refuses_missing_credential_scope() {
    let mut row =
        mgc_config::registry::Registry::new("fixture".into(), "https://registry.example/".into());
    row.token = Some("synthetic-fixture".into());
    assert!(
        ScopedRegistryClient::new(&mgc_config::chain::ScopedRegistry {
            registry: row,
            credential_scope: None
        })
        .is_err()
    );
}

// The registry retains scoped transports for metadata and both artifact modes.
// Kiểm constructor native và request helper, không chỉ transport đứng riêng.
#[test]
fn registry_scoped_chain_preserves_scope_for_metadata_and_artifacts() {
    let entries = [
        ("https://registry.example/team/", "team-token"),
        ("https://registry.example/team/private/", "private-token"),
    ]
    .into_iter()
    .map(|(url, token)| {
        let mut registry = mgc_config::registry::Registry::new("fixture".into(), url.into());
        registry.token = Some(token.into());
        mgc_config::chain::ScopedRegistry {
            registry,
            credential_scope: Some(reqwest::Url::parse(url).unwrap()),
        }
    })
    .collect();
    let registry = NpmRegistry::new_with_scoped_chain(entries).unwrap();
    for streaming in [false, true] {
        for (path, expected) in [
            ("/team/pkg", Some("Bearer team-token")),
            ("/team/private/pkg", Some("Bearer private-token")),
            ("/foreign/pkg", None),
        ] {
            let request = registry
                .request_for(&format!("https://registry.example{path}"), streaming)
                .unwrap()
                .build()
                .unwrap();
            assert_eq!(
                request
                    .headers()
                    .get(reqwest::header::AUTHORIZATION)
                    .map(|value| value.to_str().unwrap()),
                expected
            );
        }
    }
    let request = registry
        .metadata_request_for(
            "https://registry.example/team/",
            Some("ignored-scalar"),
            "https://registry.example/foreign/pkg",
        )
        .unwrap()
        .build()
        .unwrap();
    assert!(
        !request
            .headers()
            .contains_key(reqwest::header::AUTHORIZATION)
    );
    assert!(
        registry
            .request_for("https://user:password@registry.example/team/pkg", false)
            .is_err()
    );
}

// Empty or malformed chains cannot silently downgrade to legacy auth.
// Từ chối trước network nếu caller làm mất scope typed.
#[test]
fn registry_scoped_chain_rejects_empty_or_missing_scope() {
    assert!(NpmRegistry::new_with_scoped_chain(Vec::new()).is_err());
    let mut registry = mgc_config::registry::Registry::new(
        "fixture".into(),
        "https://registry.example/team/".into(),
    );
    registry.token = Some("synthetic-fixture".into());
    assert!(
        NpmRegistry::new_with_scoped_chain(vec![mgc_config::chain::ScopedRegistry {
            registry,
            credential_scope: None
        }])
        .is_err()
    );
}

// Equal scopes cannot pick a different token according to chain order.
// Chặn cấu hình mâu thuẫn trước launch, không đoán token theo thứ tự fallback.
#[test]
fn registry_scoped_chain_refuses_conflicting_credentials() {
    let entries = ["first", "second"]
        .into_iter()
        .enumerate()
        .map(|(index, token)| {
            let mut registry = mgc_config::registry::Registry::new(
                "fixture".into(),
                format!("https://registry.example/team/{index}"),
            );
            registry.token = Some(token.into());
            mgc_config::chain::ScopedRegistry {
                registry,
                credential_scope: Some(
                    reqwest::Url::parse("https://registry.example/team/").unwrap(),
                ),
            }
        })
        .collect();
    assert!(NpmRegistry::new_with_scoped_chain(entries).is_err());
}

const MAX_SCOPED_FIXTURE_HEADER_BYTES: usize = 16 * 1024;
// Build typed credentials for transport regressions only.
// Fixture dùng token tổng hợp, không đọc credential trên máy.
fn scoped_download_fixture_registry() -> NpmRegistry {
    let mut registry = mgc_config::registry::Registry::new(
        "fixture".into(),
        "https://registry.example/team/".into(),
    );
    registry.token = Some("synthetic-fixture".into());
    NpmRegistry::new_with_scoped_chain(vec![mgc_config::chain::ScopedRegistry {
        registry,
        credential_scope: Some(reqwest::Url::parse("https://registry.example/team/").unwrap()),
    }])
    .unwrap()
}
// Preserve long artifact timeout on unauthenticated streaming requests.
// Artifact ngoài scope vẫn tải đủ thời gian mà không nhận token.
#[test]
fn public_streaming_preserves_download_timeout() {
    let registry = scoped_download_fixture_registry();
    let request = registry
        .request_for("https://registry.example/public/pkg.tgz", true)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        request.timeout(),
        Some(&Duration::from_secs(SCOPED_REGISTRY_TIMEOUT_SECONDS))
    );
    assert!(
        !request
            .headers()
            .contains_key(reqwest::header::AUTHORIZATION)
    );
}
// Cloned download tasks inherit immutable scope and age mode.
// Clone không được trở về auth scalar hoặc reset policy đang bật.
#[test]
fn clone_preserves_scope_and_age_mode() {
    let registry = scoped_download_fixture_registry();
    registry.set_age_gate_armed(true);
    let clone = registry.clone_for_downloads();
    assert!(clone.age_gate_armed());
    for (path, authorized) in [("/team/pkg", true), ("/foreign/pkg", false)] {
        let request = clone
            .request_for(&format!("https://registry.example{path}"), true)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(
            request
                .headers()
                .contains_key(reqwest::header::AUTHORIZATION),
            authorized
        );
    }
}

// Exercise metadata, conditional metadata, buffer, stream, auto and cloned tasks.
// Chạy request thật qua NpmRegistry; scope phải còn sau clone và mọi kiểu tải.
#[tokio::test]
async fn registry_network_preserves_scope_across_request_paths() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    const PORTS: &[u16] = &[5314, 5341, 5413, 5431, 5134, 5143, 1534, 1543];
    const TEST_TIMEOUT_SECONDS: u64 = 10;
    let mut bound = None;
    for port in PORTS {
        if let Ok(listener) =
            tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, *port)).await
        {
            bound = Some(listener);
            break;
        }
    }
    let listener = bound.expect("a permitted fixture port must be available");
    let origin = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let server = tokio::spawn(async move {
        for (path, auth) in [
            ("/team/pkg", true),
            ("/team/pkg", true),
            ("/team/pkg.tgz", true),
            ("/foreign/pkg.tgz", false),
            ("/team/pkg.tgz", true),
            ("/team/large.tgz", true),
            ("/foreign/pkg.tgz", false),
        ] {
            let (mut stream, _) =
                tokio::time::timeout(Duration::from_secs(TEST_TIMEOUT_SECONDS), listener.accept())
                    .await
                    .unwrap()
                    .unwrap();
            let mut request = Vec::new();
            while !request.windows(4).any(|window| window == b"\r\n\r\n") {
                let mut buffer = [0; 4096];
                let read = stream.read(&mut buffer).await.unwrap();
                assert!(
                    read > 0 && request.len() < MAX_SCOPED_FIXTURE_HEADER_BYTES,
                    "request headers must be bounded"
                );
                request.extend_from_slice(&buffer[..read]);
            }
            let request = String::from_utf8(request).unwrap();
            assert_eq!(
                request.lines().next().unwrap().split_whitespace().nth(1),
                Some(path)
            );
            assert_eq!(
                request
                    .to_ascii_lowercase()
                    .contains("authorization: bearer synthetic-fixture"),
                auth
            );
            let conditional = request.to_ascii_lowercase().contains("if-none-match:");
            let status = if conditional {
                "304 Not Modified"
            } else {
                "200 OK"
            };
            let body = if conditional {
                String::new()
            } else if path == "/team/pkg" {
                r#"{"name":"pkg","description":null,"versions":{},"dist-tags":{}}"#.to_string()
            } else if path == "/team/large.tgz" {
                "x".repeat(LARGE_PKG_THRESHOLD_BYTES as usize + 1)
            } else {
                "fixture-bytes".to_string()
            };
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nETag: fixture-etag\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        }
    });
    let mut row = mgc_config::registry::Registry::new("fixture".into(), format!("{origin}/team/"));
    row.token = Some("synthetic-fixture".into());
    let registry = NpmRegistry::new_with_scoped_chain(vec![mgc_config::chain::ScopedRegistry {
        credential_scope: Some(reqwest::Url::parse(&row.url).unwrap()),
        registry: row,
    }])
    .unwrap();
    let (metadata, etag) = registry.fetch_metadata_with_etag("pkg").await.unwrap();
    assert_eq!(metadata.name, "pkg");
    assert!(
        registry
            .fetch_metadata_conditional("pkg", etag.as_deref())
            .await
            .unwrap()
            .is_none()
    );
    assert_eq!(
        registry
            .download_tarball(&format!("{origin}/team/pkg.tgz"))
            .await
            .unwrap(),
        b"fixture-bytes"
    );
    let dest = std::env::temp_dir().join(format!(
        "mgc-scoped-network-{}-{}.tgz",
        std::process::id(),
        scoped_network_fixture_port(&origin)
    ));
    let integrity = registry
        .download_tarball_to_file(&format!("{origin}/foreign/pkg.tgz"), &dest)
        .await
        .unwrap();
    assert!(integrity.starts_with("sha512-"));
    assert_eq!(std::fs::read(&dest).unwrap(), b"fixture-bytes");
    std::fs::remove_file(&dest).unwrap();
    match registry
        .download_tarball_auto(&format!("{origin}/team/pkg.tgz"), &dest)
        .await
        .unwrap()
    {
        DownloadedTarball::Bytes(bytes) => assert_eq!(bytes, b"fixture-bytes"),
        DownloadedTarball::Streamed { .. } => panic!("small fixture must be buffered"),
    }
    match registry
        .download_tarball_auto(&format!("{origin}/team/large.tgz"), &dest)
        .await
        .unwrap()
    {
        DownloadedTarball::Streamed {
            computed_integrity,
            bytes_len,
        } => {
            assert_eq!(bytes_len, LARGE_PKG_THRESHOLD_BYTES + 1);
            assert!(computed_integrity.starts_with("sha512-"));
            assert_eq!(std::fs::metadata(&dest).unwrap().len(), bytes_len);
            std::fs::remove_file(&dest).unwrap();
        }
        DownloadedTarball::Bytes(_) => panic!("large fixture must be streamed"),
    }
    assert_eq!(
        registry
            .clone_for_downloads()
            .download_tarball(&format!("{origin}/foreign/pkg.tgz"))
            .await
            .unwrap(),
        b"fixture-bytes"
    );
    server.await.unwrap();
}
fn scoped_network_fixture_port(origin: &str) -> u16 {
    reqwest::Url::parse(origin).unwrap().port().unwrap()
}

// A rejected credential redirect must not disclose private names through fallback.
// Dùng NpmRegistry thật để chứng minh fallback không nhận request sau scope refusal.
#[tokio::test]
async fn scoped_metadata_redirect_refusal_does_not_fallback() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    const PORTS: &[u16] = &[5314, 5341, 5413, 5431, 5134, 5143, 1534, 1543];
    const EXTRA_REQUEST_TIMEOUT: Duration = Duration::from_millis(250);
    const REQUEST_TIMEOUT_SECONDS: u64 = 10;
    let mut bound = None;
    for port in PORTS {
        if let Ok(listener) =
            tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, *port)).await
        {
            bound = Some(listener);
            break;
        }
    }
    let listener = bound.expect("a permitted fixture port must be available");
    let origin = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let server_origin = origin.clone();
    let server = tokio::spawn(async move {
        for _ in 0..4 {
            let (mut stream, _) = tokio::time::timeout(
                Duration::from_secs(REQUEST_TIMEOUT_SECONDS),
                listener.accept(),
            )
            .await
            .unwrap()
            .unwrap();
            let mut request = Vec::new();
            while !request.windows(4).any(|window| window == b"\r\n\r\n") {
                let mut buffer = [0; 4096];
                let read = stream.read(&mut buffer).await.unwrap();
                assert!(
                    read > 0 && request.len() < MAX_SCOPED_FIXTURE_HEADER_BYTES,
                    "request headers must be bounded"
                );
                request.extend_from_slice(&buffer[..read]);
            }
            let request = String::from_utf8(request).unwrap();
            assert_eq!(
                request.lines().next().unwrap().split_whitespace().nth(1),
                Some("/team/private-pkg")
            );
            assert!(
                request
                    .to_ascii_lowercase()
                    .contains("authorization: bearer synthetic-fixture")
            );
            let response = format!(
                "HTTP/1.1 302 Found\r\nLocation: {server_origin}/foreign/pkg\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        }
        assert!(
            tokio::time::timeout(EXTRA_REQUEST_TIMEOUT, listener.accept())
                .await
                .is_err(),
            "neither redirect target nor fallback may receive a request"
        );
    });
    let mut primary =
        mgc_config::registry::Registry::new("private".into(), format!("{origin}/team/"));
    primary.token = Some("synthetic-fixture".into());
    let primary = mgc_config::chain::ScopedRegistry {
        credential_scope: Some(reqwest::Url::parse(&primary.url).unwrap()),
        registry: primary,
    };
    let fallback = mgc_config::chain::ScopedRegistry {
        registry: mgc_config::registry::Registry::new(
            "fallback".into(),
            format!("{origin}/fallback/"),
        ),
        credential_scope: None,
    };
    let registry = NpmRegistry::new_with_scoped_chain(vec![primary, fallback]).unwrap();
    let error = registry
        .fetch_metadata_with_etag("private-pkg")
        .await
        .unwrap_err();
    assert!(error.chain().any(|cause| {
        cause
            .downcast_ref::<reqwest::Error>()
            .is_some_and(|error| error.is_redirect())
    }));
    server.await.unwrap();
}
