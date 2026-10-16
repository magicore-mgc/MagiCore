#![allow(clippy::unwrap_used)]

// ArtifactFetcher integrity and resolved-URL regressions through real HTTP/CAS.
// Kiểm API fetch riêng bằng process cô lập, không dùng credential/cache của user.
use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};
use mgc_types::capabilities::ArtifactFetcher;
use mgc_types::{PackageId, PackageName, Version};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const FIXTURE_PORTS: &[u16] = &[
    4315, 4351, 4135, 4153, 4513, 4531, 3415, 3451, 3145, 3154, 3541, 3514, 1345, 1354, 1435, 1453,
    1534, 1543, 5134, 5143, 5314, 5341, 5413, 5431,
];
const FIXTURE_ACCEPT_TIMEOUT_SECONDS: u64 = 10;
const MAX_FIXTURE_HEADER_BYTES: usize = 16 * 1024;
const PROBE_CHILD_ENV: &str = "MGC_ARTIFACT_FETCH_PROBE_CHILD";
const FIXTURE_BODY: &[u8] = b"altered-artifact-body";

// Set fixture policy before the child's runtime exists; never mutate process env.
// Mỗi probe có HOME/data/tmp riêng và flag loopback rõ, không race env giữa test.
fn run_isolated_probe<F: std::future::Future<Output = ()>>(name: &str, probe: F) {
    if std::env::var(PROBE_CHILD_ENV).as_deref() == Ok(name) {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("create artifact probe runtime")
            .block_on(probe);
        return;
    }
    let sandbox = tempfile::Builder::new()
        .prefix("mgc-artifact-fetch-")
        .tempdir()
        .unwrap();
    let home = sandbox.path().join("home");
    let temporary = sandbox.path().join("tmp");
    std::fs::create_dir_all(&home).unwrap();
    std::fs::create_dir_all(&temporary).unwrap();
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--test-threads=1", "--nocapture"])
        .env(PROBE_CHILD_ENV, name)
        .env("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1")
        .env("MGC_STRICT_INTEGRITY", "1")
        .env_remove("MAGICORE_WEB_REGISTRY_URL")
        .env_remove("MAGICORE_WEB_REGISTRY_TOKEN")
        .env_remove("MAGICORE_WEB_ALLOWED_REGISTRIES")
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .env("MGC_DATA_DIR", sandbox.path().join("data"))
        .env("MGC_CACHE_DIR", sandbox.path().join("cache"))
        .env("MGC_STORE_DIR", sandbox.path().join("store"))
        .env("TMPDIR", &temporary)
        .env("TMP", &temporary)
        .env("TEMP", &temporary)
        .output()
        .expect("run isolated artifact probe");
    assert!(
        output.status.success(),
        "isolated artifact probe failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

async fn artifact_fixture() -> (String, tokio::task::JoinHandle<String>) {
    let mut selected = None;
    for port in FIXTURE_PORTS {
        if let Ok(listener) =
            tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, *port)).await
        {
            selected = Some(listener);
            break;
        }
    }
    let listener = selected.expect("a permitted artifact fixture port must be available");
    let origin = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let server = tokio::spawn(async move {
        let (mut stream, _) = tokio::time::timeout(
            std::time::Duration::from_secs(FIXTURE_ACCEPT_TIMEOUT_SECONDS),
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
                read > 0 && request.len() < MAX_FIXTURE_HEADER_BYTES,
                "fixture headers must be bounded"
            );
            request.extend_from_slice(&buffer[..read]);
        }
        let path = String::from_utf8(request)
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .to_owned();
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            FIXTURE_BODY.len()
        );
        stream.write_all(header.as_bytes()).await.unwrap();
        stream.write_all(FIXTURE_BODY).await.unwrap();
        path
    });
    (origin, server)
}

fn fixture_graph(url: String, integrity: &str) -> ResolvedGraph {
    ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("fixture").unwrap(),
                Version::parse("1.0.0").unwrap(),
            ),
            integrity: integrity.to_string(),
            tarball_url: url,
            deps: Vec::new(),
            peer_deps: Vec::new(),
            direct: true,
            dev: false,
        }],
    }
}

fn fixture_integrity() -> String {
    use base64::Engine;
    use sha2::Digest;
    format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(sha2::Sha512::digest(FIXTURE_BODY))
    )
}

async fn assert_artifact_store_result(integrity: &str, expected_accept: bool) {
    let (origin, server) = artifact_fixture().await;
    let root = tempfile::Builder::new()
        .prefix("mgc-artifact-cas-")
        .tempdir()
        .unwrap();
    let store = mgc_store::ContentStore::new(root.path().join("cas")).unwrap();
    let hash = mgc_store::IntegrityHash::from_bytes(FIXTURE_BODY, false);
    let adapter = mgc_web_adapter::WebAdapter::with_registry(origin.clone())
        .unwrap()
        .with_store(store.clone());
    let result = adapter
        .fetch(&fixture_graph(
            format!("{origin}/fixture-custom.tgz"),
            integrity,
        ))
        .await;
    let _ = server.await.unwrap();
    assert_eq!(
        result.is_ok(),
        expected_accept,
        "artifact acceptance must follow declared integrity"
    );
    assert_eq!(
        store.contains(&hash),
        expected_accept,
        "CAS must only contain verified artifact bytes"
    );
}

#[test]
fn altered_artifact_is_rejected_before_cas_import() {
    run_isolated_probe("altered_artifact_is_rejected_before_cas_import", async {
        assert_artifact_store_result("sha512-AAAA", false).await;
    });
}

#[test]
fn matching_integrity_is_accepted_control() {
    run_isolated_probe("matching_integrity_is_accepted_control", async {
        assert_artifact_store_result(&fixture_integrity(), true).await;
    });
}

#[test]
fn strict_missing_integrity_is_rejected_before_cas_import() {
    run_isolated_probe(
        "strict_missing_integrity_is_rejected_before_cas_import",
        async {
            assert_artifact_store_result("", false).await;
        },
    );
}

#[test]
fn resolved_tarball_url_is_used() {
    run_isolated_probe("resolved_tarball_url_is_used", async {
        let (origin, server) = artifact_fixture().await;
        let adapter = mgc_web_adapter::WebAdapter::with_registry(origin.clone()).unwrap();
        adapter
            .fetch(&fixture_graph(
                format!("{origin}/fixture-custom.tgz"),
                &fixture_integrity(),
            ))
            .await
            .unwrap();
        assert_eq!(server.await.unwrap(), "/fixture-custom.tgz");
    });
}
