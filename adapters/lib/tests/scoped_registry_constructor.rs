#![allow(clippy::unwrap_used)]

// Native prefetch/fetch preserve path-scoped credentials through download tasks.
// Kiểm helper thực qua HTTP và cache riêng; không mutate env runtime hay dùng token user.
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
const PROBE_CHILD_ENV: &str = "MGC_PREFETCH_CREDENTIAL_PROBE_CHILD";
const FIXTURE_BODY: &[u8] = b"fixture-prefetch-bytes";

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

async fn assert_lib_fetch_scope(path: &str, expected_auth: bool) {
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
            FIXTURE_BODY.len()
        );
        stream.write_all(header.as_bytes()).await.unwrap();
        stream.write_all(FIXTURE_BODY).await.unwrap();
        (observed_path, observed_auth)
    });
    let mut row = mgc_config::registry::Registry::new("fixture".into(), format!("{origin}/team/"));
    row.token = Some("synthetic-fixture".into());
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("package.json"),
        r#"{"name":"scope-lib","version":"1.0.0","type":"module"}"#,
    )
    .unwrap();
    let adapter = mgc_lib_adapter::adapter_for_with_scoped_chain(
        project.path(),
        vec![mgc_config::chain::ScopedRegistry {
            credential_scope: Some(row.url.parse().unwrap()),
            registry: row,
        }],
    )
    .unwrap()
    .unwrap();
    assert_eq!(adapter.language(), "ts");
    let pkg = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("fixture").unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity: mgc_web_adapter::lockfile::compute_tarball_integrity(FIXTURE_BODY),
        tarball_url: format!("{origin}{path}"),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    };
    adapter
        .fetch(&ResolvedGraph {
            packages: vec![pkg],
        })
        .await
        .unwrap();
    let (observed_path, observed_auth) = server.await.unwrap();
    assert_eq!(observed_path, path);
    assert_eq!(
        observed_auth, expected_auth,
        "native fetch must retain the configured credential path scope"
    );
}

#[test]
fn lib_private_path_retains_credentials() {
    run_isolated_probe("lib_private_path_retains_credentials", async {
        assert_lib_fetch_scope("/team/pkg.tgz", true).await;
    });
}

#[test]
fn lib_foreign_path_has_no_credentials() {
    run_isolated_probe("lib_foreign_path_has_no_credentials", async {
        assert_lib_fetch_scope("/foreign/pkg.tgz", false).await;
    });
}

// Non-npm lanes must not interpret an unrelated npm credential chain.
// Đối chứng construction với endpoint npm sai để phát hiện coupling ngoài TS.
async fn assert_non_ts_isolation(file: &str, manifest: &str, language: &str) {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join(file), manifest).unwrap();
    let registry = mgc_config::registry::Registry::new(
        "invalid-unused-npm".into(),
        "invalid-npm-endpoint".into(),
    );
    let adapter = mgc_lib_adapter::adapter_for_with_scoped_chain(
        project.path(),
        vec![mgc_config::chain::ScopedRegistry {
            registry,
            credential_scope: None,
        }],
    )
    .unwrap()
    .unwrap();
    assert_eq!(adapter.language(), language);
    assert!(
        adapter.probe_artifact_fetcher().is_err(),
        "non-TS must retain standalone fetch refusal"
    );
}

#[test]
fn lib_rust_ignores_unused_npm_chain() {
    run_isolated_probe("lib_rust_ignores_unused_npm_chain", async {
        assert_non_ts_isolation(
            "Cargo.toml",
            "[package]\nname=\"fixture\"\nversion=\"1.0.0\"\n",
            "rust",
        )
        .await;
    });
}

#[test]
fn lib_python_ignores_unused_npm_chain() {
    run_isolated_probe("lib_python_ignores_unused_npm_chain", async {
        assert_non_ts_isolation(
            "pyproject.toml",
            "[project]\nname=\"fixture\"\nversion=\"1.0.0\"\n",
            "python",
        )
        .await;
    });
}

#[test]
fn lib_go_ignores_unused_npm_chain() {
    run_isolated_probe("lib_go_ignores_unused_npm_chain", async {
        assert_non_ts_isolation("go.mod", "module example.test/fixture\ngo 1.23\n", "go").await;
    });
}

#[test]
fn lib_java_ignores_unused_npm_chain() {
    run_isolated_probe("lib_java_ignores_unused_npm_chain", async {
        assert_non_ts_isolation("pom.xml", "<project><modelVersion>4.0.0</modelVersion><groupId>test</groupId><artifactId>fixture</artifactId><version>1.0.0</version></project>", "java").await;
    });
}

#[test]
fn lib_dotnet_ignores_unused_npm_chain() {
    run_isolated_probe("lib_dotnet_ignores_unused_npm_chain", async {
        assert_non_ts_isolation("Fixture.csproj", "<Project Sdk=\"Microsoft.NET.Sdk\"><PropertyGroup><TargetFramework>net8.0</TargetFramework></PropertyGroup></Project>", "dotnet").await;
    });
}
