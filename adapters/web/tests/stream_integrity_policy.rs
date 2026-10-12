#![allow(clippy::unwrap_used)]

// Native prefetch/fetch preserve path-scoped credentials through download tasks.
// Kiểm helper thực qua HTTP và cache riêng; không mutate env runtime hay dùng token user.
use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};
use mgc_types::{PackageId, PackageName, Version};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const FIXTURE_PORTS: &[u16] = &[
    4315, 4351, 4135, 4153, 4513, 4531, 3415, 3451, 3145, 3154, 3541, 3514, 1345, 1354, 1435, 1453,
    1534, 1543, 5134, 5143, 5314, 5341, 5413, 5431,
];
const FIXTURE_ACCEPT_TIMEOUT_SECONDS: u64 = 10;
const MAX_FIXTURE_HEADER_BYTES: usize = 16 * 1024;
const PROBE_CHILD_ENV: &str = "MGC_PREFETCH_CREDENTIAL_PROBE_CHILD";

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

async fn assert_stream_sri(prefetch: bool, mode: &str) {
    let missing = mode == "missing";
    let expected_accept = !matches!(mode, "missing" | "wrong");
    let path = "/team/pkg.tgz";
    let body =
        vec![b'x'; mgc_web_adapter::native::npm_registry::LARGE_PKG_THRESHOLD_BYTES as usize + 1];
    let server_body = body.clone();
    let mut selected = None;
    for port in FIXTURE_PORTS {
        if let Ok(listener) =
            tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, *port)).await
        {
            selected = Some(listener);
            break;
        }
    }
    let listener = selected.expect("a permitted credential fixture port must be available");
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
            let count = stream.read(&mut buffer).await.unwrap();
            assert!(
                count > 0 && request.len() < MAX_FIXTURE_HEADER_BYTES,
                "fixture headers must be bounded"
            );
            request.extend_from_slice(&buffer[..count]);
        }
        let request = String::from_utf8(request).unwrap();
        let line = request.lines().next().unwrap();
        assert_eq!(
            line.split_whitespace().next(),
            Some("GET"),
            "native fetch must use one GET, not an unscoped HEAD"
        );
        let observed_path = line.split_whitespace().nth(1).unwrap().to_owned();
        let observed_auth = request.to_ascii_lowercase().contains("authorization:");
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            server_body.len()
        );
        stream.write_all(header.as_bytes()).await.unwrap();
        stream.write_all(&server_body).await.unwrap();
        (observed_path, observed_auth)
    });
    let mut row = mgc_config::registry::Registry::new("fixture".into(), format!("{origin}/team/"));
    row.token = Some("synthetic-fixture".into());
    let registry = mgc_web_adapter::native::npm_registry::NpmRegistry::new_with_scoped_chain(vec![
        mgc_config::chain::ScopedRegistry {
            credential_scope: Some(row.url.parse().unwrap()),
            registry: row,
        },
    ])
    .unwrap();
    let pkg = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("fixture").unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity: match mode {
            "missing" => String::new(),
            "wrong" => "sha512-invalid sha256-invalid".to_string(),
            "sha512" => mgc_web_adapter::lockfile::compute_tarball_integrity(&body),
            "multi" => format!(
                "sha256-invalid {}",
                mgc_web_adapter::lockfile::compute_tarball_integrity(&body)
            ),
            _ => format!(
                "sha256-{}",
                mgc_web_adapter::install::integrity::compute_sha256_b64_str(&body)
            ),
        },
        tarball_url: format!("{origin}{path}"),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    };
    let root = tempfile::Builder::new()
        .prefix("mgc-native-scope-cache-")
        .tempdir()
        .unwrap();
    let cache = mgc_store::PackageCache::new(root.path().join("cache")).unwrap();
    let accepted = if prefetch {
        mgc_web_adapter::install::download::prefetch_tarballs(
            &ResolvedGraph {
                packages: vec![pkg.clone()],
            },
            &std::collections::HashSet::new(),
            &cache,
            None,
            &registry,
        )
        .await
        .is_ok()
    } else {
        mgc_web_adapter::install::fetch::get_tarball_bytes_with_registry(
            &pkg,
            &cache,
            None,
            Some(&registry),
            &tokio::sync::Semaphore::new(1),
        )
        .await
        .is_ok()
    };
    let (observed_path, observed_auth) = server.await.unwrap();
    assert_eq!(observed_path, path);
    assert!(
        observed_auth,
        "native fetch must retain the configured credential path scope"
    );
    eprintln!(
        "streamed=true missing_sri={} accepted={} cache_published={}",
        missing,
        accepted,
        cache.get_tarball(&pkg.id).unwrap().is_some()
    );
    assert_eq!(
        accepted, expected_accept,
        "streamed fetch must deny missing strict SRI and accept valid SHA256"
    );
    if !expected_accept {
        assert!(
            cache.get_tarball(&pkg.id).unwrap().is_none(),
            "unverified streamed artifact must not enter cache"
        );
    } else {
        assert_eq!(cache.get_tarball(&pkg.id).unwrap().unwrap(), body);
    }
}

#[test]
fn prefetch_stream_sha256() {
    run_isolated_probe("prefetch_stream_sha256", async {
        assert_stream_sri(true, "sha256").await;
    });
}

#[test]
fn fetch_stream_sha256() {
    run_isolated_probe("fetch_stream_sha256", async {
        assert_stream_sri(false, "sha256").await;
    });
}

#[test]
fn prefetch_stream_strict_missing() {
    run_isolated_probe("prefetch_stream_strict_missing", async {
        assert_stream_sri(true, "missing").await;
    });
}

#[test]
fn fetch_stream_strict_missing() {
    run_isolated_probe("fetch_stream_strict_missing", async {
        assert_stream_sri(false, "missing").await;
    });
}

#[test]
fn prefetch_stream_sha512() {
    run_isolated_probe("prefetch_stream_sha512", async {
        assert_stream_sri(true, "sha512").await;
    });
}

#[test]
fn prefetch_stream_multi() {
    run_isolated_probe("prefetch_stream_multi", async {
        assert_stream_sri(true, "multi").await;
    });
}

#[test]
fn prefetch_stream_wrong() {
    run_isolated_probe("prefetch_stream_wrong", async {
        assert_stream_sri(true, "wrong").await;
    });
}

#[test]
fn fetch_stream_sha512() {
    run_isolated_probe("fetch_stream_sha512", async {
        assert_stream_sri(false, "sha512").await;
    });
}

#[test]
fn fetch_stream_multi() {
    run_isolated_probe("fetch_stream_multi", async {
        assert_stream_sri(false, "multi").await;
    });
}

#[test]
fn fetch_stream_wrong() {
    run_isolated_probe("fetch_stream_wrong", async {
        assert_stream_sri(false, "wrong").await;
    });
}
