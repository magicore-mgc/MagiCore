//! Crates protocol engine tests — hermetic via mockito (no real network).
//! Test engine crates — hermetic qua mockito (không mạng thật).

#![allow(clippy::unwrap_used)]

use mgc_resolver::protocols::sha256_hex;
use mgc_resolver::protocols::{CratesProtocol, RegistryProtocol, RegistryRoot};

#[tokio::test]
async fn crates_missing_index_checksum_fails_resolution() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    server
        .mock("GET", "/se/rd/serde")
        .with_status(200)
        .with_body(crate_line("serde", "1.0.0", false, "", ""))
        .create_async()
        .await;

    let error = CratesProtocol::with_download_base(&server.url(), &server.url())
        .resolve("serde", "^1.0")
        .await
        .expect_err("crates.io index entry without cksum must not resolve");
    assert!(error.to_string().contains("cksum"), "{error}");
}

async fn mock_server() -> Option<mockito::ServerGuard> {
    match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => drop(listener),
        Err(error) => panic!(
            "crates.io mock tests require localhost; refusing to report skipped tests as passing: {error}"
        ),
    }
    Some(mockito::Server::new_async().await)
}

/// Sparse-index NDJSON line for one crate version — genuine index shape
/// (BARE 64-hex `cksum`, no scheme prefix, as served by index.crates.io).
fn crate_line(name: &str, vers: &str, yanked: bool, cksum: &str, deps: &str) -> String {
    format!(
        r#"{{"name":"{name}","vers":"{vers}","deps":[{deps}],"cksum":"{cksum}","features":{{}},"yanked":{yanked},"links":null}}"#
    )
}

fn crate_line_with_features(
    name: &str,
    vers: &str,
    cksum: &str,
    deps: &str,
    features: &str,
    features2: &str,
) -> String {
    format!(
        r#"{{"name":"{name}","vers":"{vers}","deps":[{deps}],"cksum":"{cksum}","features":{features},"features2":{features2},"yanked":false,"links":null}}"#
    )
}

fn dep(name: &str, req: &str, kind: &str, optional: bool) -> String {
    format!(
        r#"{{"name":"{name}","req":"{req}","features":[],"optional":{optional},"default_features":true,"target":null,"kind":"{kind}","package":null}}"#
    )
}

#[tokio::test]
async fn crates_retries_a_truncated_compressed_index_body() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let index_body = crate_line("retrycrate", "1.0.0", false, &sha256_hex(b"retrycrate"), "");
    let server = tokio::spawn(async move {
        for attempt in 0..2 {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut chunk = [0_u8; 1024];
            loop {
                let read = socket.read(&mut chunk).await.unwrap();
                assert_ne!(read, 0, "client closed before sending request headers");
                request.extend_from_slice(&chunk[..read]);
                if request.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }

            if attempt == 0 {
                socket
                    .write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: 3\r\nConnection: close\r\n\r\nbad",
                    )
                    .await
                    .unwrap();
            } else {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    index_body.len(),
                    index_body
                );
                socket.write_all(response.as_bytes()).await.unwrap();
            }
        }
        2
    });

    let base = format!("http://{address}");
    let protocol = CratesProtocol::with_download_base(&base, &base);
    let entries = protocol.resolve_graph("retrycrate", "1").await.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "retrycrate");
    assert_eq!(server.await.unwrap(), 2);
}

#[tokio::test]
async fn crates_resolves_root_features_and_honors_disabled_default_features() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let root_bytes = b"ESP_HAL_ROOT";
    let chip_bytes = b"ESP_HAL_CHIP_CORE";
    let runtime_bytes = b"ESP_HAL_CHIP_RUNTIME";
    let default_bytes = b"ESP_HAL_DEFAULT_EXTRA";
    let root_cksum = sha256_hex(root_bytes);
    let chip_cksum = sha256_hex(chip_bytes);
    let runtime_cksum = sha256_hex(runtime_bytes);
    let default_cksum = sha256_hex(default_bytes);
    let dependencies = format!(
        "{},{}",
        dep("chip-core", "^1", "normal", true),
        dep("default-extra", "^1", "normal", true)
    );

    server
        .mock("GET", "/es/p-/esp-hal")
        .with_status(200)
        .with_body(crate_line_with_features(
            "esp-hal",
            "1.2.2",
            &root_cksum,
            &dependencies,
            r#"{"default":["dep:default-extra"],"esp32c3":["dep:chip-core"]}"#,
            "{}",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/de/fa/default-extra")
        .with_status(200)
        .with_body(crate_line(
            "default-extra",
            "1.0.0",
            false,
            &default_cksum,
            "",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/ch/ip/chip-core")
        .with_status(200)
        .with_body(crate_line_with_features(
            "chip-core",
            "1.0.0",
            &chip_cksum,
            &format!(
                "{},{}",
                dep("chip-runtime", "^1", "normal", false),
                dep("chip-extra", "^1", "normal", true)
            ),
            "{}",
            "{}",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/ch/ip/chip-runtime")
        .with_status(200)
        .with_body(crate_line(
            "chip-runtime",
            "1.0.0",
            false,
            &runtime_cksum,
            "",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/ch/ip/chip-extra")
        .with_status(200)
        .with_body(crate_line(
            "chip-extra",
            "1.0.0",
            false,
            &sha256_hex(b"ESP_HAL_CHIP_EXTRA"),
            "",
        ))
        .create_async()
        .await;

    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let entries = protocol
        .resolve_graph_roots_with_options(&[RegistryRoot {
            name: "esp-hal".to_string(),
            range: "1.2.2".to_string(),
            features: vec!["esp32c3".to_string()],
            default_features: false,
        }])
        .await
        .unwrap();

    assert_eq!(entries.len(), 3);
    let root = entries
        .iter()
        .find(|entry| entry.name == "esp-hal")
        .unwrap();
    assert_eq!(root.deps, [("chip-core".to_string(), "=1.0.0".to_string())]);
    assert!(entries.iter().any(|entry| entry.name == "chip-core"));
    let chip_core = entries
        .iter()
        .find(|entry| entry.name == "chip-core")
        .unwrap();
    assert_eq!(
        chip_core.deps,
        [("chip-runtime".to_string(), "=1.0.0".to_string())]
    );
    assert!(entries.iter().any(|entry| entry.name == "chip-runtime"));
    assert!(!entries.iter().any(|entry| entry.name == "chip-extra"));
    assert!(!entries.iter().any(|entry| entry.name == "default-extra"));
    assert!(
        root.extra_markers
            .iter()
            .any(|marker| marker == "optional:default-extra"),
        "disabled default feature must keep its optional edge dormant"
    );
}

#[tokio::test]
async fn crates_resolves_multiple_versions_of_a_transitive_crate() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let left_edge = dep("embassy-sync", "^0.6", "normal", false);
    let right_edge = dep("embassy-sync", "^0.8", "normal", false);
    server
        .mock("GET", "/ro/ot/root")
        .with_status(200)
        .with_body(crate_line(
            "root",
            "1.0.0",
            false,
            &sha256_hex(b"root"),
            &format!(
                "{},{}",
                dep("left", "^1", "normal", false),
                dep("right", "^1", "normal", false)
            ),
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/le/ft/left")
        .with_status(200)
        .with_body(crate_line(
            "left",
            "1.0.0",
            false,
            &sha256_hex(b"left"),
            &left_edge,
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/ri/gh/right")
        .with_status(200)
        .with_body(crate_line(
            "right",
            "1.0.0",
            false,
            &sha256_hex(b"right"),
            &right_edge,
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/em/ba/embassy-sync")
        .with_status(200)
        .with_body(
            [
                crate_line("embassy-sync", "0.6.2", false, &sha256_hex(b"sync-0.6"), ""),
                crate_line("embassy-sync", "0.7.2", false, &sha256_hex(b"sync-0.7"), ""),
                crate_line("embassy-sync", "0.8.0", false, &sha256_hex(b"sync-0.8"), ""),
            ]
            .join("\n"),
        )
        .create_async()
        .await;

    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let entries = protocol.resolve_graph("root", "^1").await.unwrap();
    assert_eq!(entries.len(), 5);
    let left = entries.iter().find(|entry| entry.name == "left").unwrap();
    let right = entries.iter().find(|entry| entry.name == "right").unwrap();
    assert_eq!(
        left.deps,
        [("embassy-sync".to_string(), "=0.6.2".to_string())]
    );
    assert_eq!(
        right.deps,
        [("embassy-sync".to_string(), "=0.8.0".to_string())]
    );
    assert!(
        entries
            .iter()
            .any(|entry| entry.name == "embassy-sync" && entry.version == "0.6.2")
    );
    assert!(
        entries
            .iter()
            .any(|entry| entry.name == "embassy-sync" && entry.version == "0.8.0")
    );
}

#[tokio::test]
async fn crates_resolves_complete_lock_closure_with_optional_and_target_edges() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let serde_bytes = b"SERDE_CRATE_1.0.219";
    let core_bytes = b"SERDE_CORE_1.0.5";
    let derive_bytes = b"SERDE_DERIVE_1.0.219";
    let windows_bytes = b"WINDOWS_SYS_0.59.0";
    let derive_core_bytes = b"DERIVE_CORE_1.0.0";
    let serde_cksum = sha256_hex(serde_bytes);
    let core_cksum = sha256_hex(core_bytes);
    let derive_cksum = sha256_hex(derive_bytes);
    let windows_cksum = sha256_hex(windows_bytes);
    let derive_core_cksum = sha256_hex(derive_core_bytes);

    // serde 1.0.219 → serde_core + dormant optional serde_derive + Windows-only
    // windows-sys. Unreferenced optional edges stay out; target-qualified
    // mandatory dependencies are resolved for every host.
    let windows_dep = r#"{"name":"windows-sys","req":"^0.59","features":[],"optional":false,"default_features":true,"target":"cfg(windows)","kind":"normal","package":null}"#;
    let serde_dependencies = format!(
        "{},{},{}",
        dep("serde_core", "1.0", "normal", false),
        dep("serde_derive", "=1.0.219", "normal", true),
        windows_dep
    );
    let serde_body = [
        crate_line("serde", "1.0.219", false, &serde_cksum, &serde_dependencies),
        crate_line("serde", "2.0.0", true, &serde_cksum, ""),
        crate_line("serde", "1.0.100", false, &serde_cksum, ""),
    ]
    .join("\n");

    server
        .mock("GET", "/se/rd/serde")
        .with_status(200)
        .with_body(serde_body)
        .create_async()
        .await;

    // serde_core 1.0.5 → no deps
    server
        .mock("GET", "/se/rd/serde_core")
        .with_status(200)
        .with_body(crate_line("serde_core", "1.0.5", false, &core_cksum, ""))
        .create_async()
        .await;
    server
        .mock("GET", "/se/rd/serde_derive")
        .with_status(200)
        .with_body(crate_line_with_features(
            "serde_derive",
            "1.0.219",
            &derive_cksum,
            &format!(
                "{},{}",
                dep("derive-core", "^1", "normal", false),
                dep("serde-extra", "^1", "normal", true)
            ),
            "{}",
            "{}",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/de/ri/derive-core")
        .with_status(200)
        .with_body(crate_line(
            "derive-core",
            "1.0.0",
            false,
            &derive_core_cksum,
            "",
        ))
        .create_async()
        .await;

    server
        .mock("GET", "/wi/nd/windows-sys")
        .with_status(200)
        .with_body(crate_line(
            "windows-sys",
            "0.59.0",
            false,
            &windows_cksum,
            "",
        ))
        .create_async()
        .await;

    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let entries = protocol.resolve_graph("serde", "^1.0").await.unwrap();

    assert_eq!(
        entries.len(),
        3,
        "unreferenced optional packages stay dormant"
    );
    let serde = entries.iter().find(|entry| entry.name == "serde").unwrap();
    assert_eq!(serde.name, "serde");
    assert_eq!(serde.version, "1.0.219");
    assert_eq!(
        serde.deps,
        vec![
            ("serde_core".to_string(), "=1.0.5".to_string()),
            ("windows-sys".to_string(), "=0.59.0".to_string()),
        ]
    );
    assert!(
        serde
            .extra_markers
            .iter()
            .any(|m| m == "optional:serde_derive"),
        "optional dep must be recorded in extras: {:?}",
        serde.extra_markers
    );

    let core = entries
        .iter()
        .find(|entry| entry.name == "serde_core")
        .unwrap();
    assert_eq!(core.name, "serde_core");
    assert_eq!(core.version, "1.0.5");
    assert!(!entries.iter().any(|entry| entry.name == "serde_derive"));
    assert!(!entries.iter().any(|entry| entry.name == "derive-core"));
    assert!(!entries.iter().any(|entry| entry.name == "serde-extra"));
    assert!(entries.iter().any(|entry| entry.name == "windows-sys"));
    assert!(
        serde
            .extra_markers
            .iter()
            .any(|marker| marker == "target:cfg(windows)")
    );
}

#[tokio::test]
async fn crates_feature_resolution_skips_dormant_optional_edges() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let checksum = |name: &str| sha256_hex(name.as_bytes());

    server
        .mock("GET", "/ro/ot/root")
        .with_status(200)
        .with_body(crate_line_with_features(
            "root",
            "1.0.0",
            &checksum("root"),
            &dep("syn", "^3", "normal", false),
            r#"{"default":["api"],"api":["syn/proc-macro","syn/weak-forward"]}"#,
            "{}",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/3/s/syn")
        .with_status(200)
        .with_body(crate_line_with_features(
            "syn",
            "3.0.0",
            &checksum("syn"),
            &format!(
                "{},{},{}",
                dep("proc-macro2", "^1", "normal", false),
                dep("quote", "^1", "normal", true),
                r#"{"name":"indexmap","req":"^2","features":["compact"],"optional":true,"default_features":true,"target":null,"kind":"normal","package":null}"#,
            ),
            r#"{"default":["derive"],"derive":["dep:quote"],"proc-macro":["proc-macro2/proc-macro"],"weak-forward":["proc-macro2?/span","indexmap?/compact"]}"#,
            "{}",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/in/de/indexmap")
        .with_status(200)
        .with_body(crate_line_with_features(
            "indexmap",
            "2.14.2",
            &checksum("indexmap"),
            &format!(
                "{},{}",
                dep("smallvec", "^1", "normal", false),
                dep("indexmap-extra", "^1", "normal", true)
            ),
            r#"{"compact":["dep:indexmap-extra"]}"#,
            "{}",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/sm/al/smallvec")
        .with_status(200)
        .with_body(crate_line(
            "smallvec",
            "1.0.0",
            false,
            &checksum("smallvec"),
            "",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/in/de/indexmap-extra")
        .with_status(200)
        .with_body(crate_line(
            "indexmap-extra",
            "1.0.0",
            false,
            &checksum("indexmap-extra"),
            "",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/pr/oc/proc-macro2")
        .with_status(200)
        .with_body(crate_line_with_features(
            "proc-macro2",
            "1.0.0",
            &checksum("proc-macro2"),
            &dep("unicode-ident", "^1", "normal", true),
            r#"{"proc-macro":[]}"#,
            r#"{"span":["dep:unicode-ident"]}"#,
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/un/ic/unicode-ident")
        .with_status(200)
        .with_body(crate_line(
            "unicode-ident",
            "1.0.0",
            false,
            &checksum("unicode-ident"),
            "",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/qu/ot/quote")
        .with_status(200)
        .with_body(crate_line("quote", "1.0.0", false, &checksum("quote"), ""))
        .create_async()
        .await;
    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let entries = protocol.resolve_graph("root", "1").await.unwrap();
    let by_name = entries
        .iter()
        .map(|entry| (entry.name.as_str(), entry))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert!(
        by_name.contains_key("quote"),
        "default feature must activate quote"
    );
    assert!(
        by_name.contains_key("proc-macro2"),
        "requested child feature must activate proc-macro2"
    );
    assert!(
        by_name.contains_key("unicode-ident"),
        "weak feature forwarding must affect a non-optional active dependency"
    );
    assert!(
        by_name.contains_key("indexmap"),
        "a weak feature reference retains an optional dependency as lock-only"
    );
    assert_eq!(
        by_name["indexmap"].deps,
        [
            ("indexmap-extra".to_string(), "=1.0.0".to_string()),
            ("smallvec".to_string(), "=1.0.0".to_string()),
        ]
    );
    assert!(
        by_name.contains_key("smallvec") && by_name.contains_key("indexmap-extra"),
        "lock-only packages retain dependencies required by edge features"
    );
    assert!(by_name["syn"].deps.iter().any(|(name, _)| name == "quote"));
    assert!(
        by_name["syn"]
            .extra_markers
            .iter()
            .any(|marker| marker == "optional:indexmap")
    );
    assert!(
        by_name["syn"]
            .deps
            .iter()
            .any(|(name, _)| name == "indexmap")
    );
}

#[tokio::test]
async fn crates_refuses_unmanaged_alternate_registry_edges() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    server
        .mock("GET", "/fo/re/foreign_root")
        .with_status(200)
        .with_body(crate_line_with_features(
            "foreign_root",
            "1.0.0",
            &sha256_hex(b"foreign-root"),
            r#"{"name":"private-crate","req":"^1","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal","package":null,"registry":"https://registry.example/index"}"#,
            "{}",
            "{}",
        ))
        .create_async()
        .await;

    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let error = protocol
        .resolve_graph("foreign_root", "1")
        .await
        .unwrap_err();
    assert!(error.to_string().contains("alternate registry"), "{error}");
}

#[tokio::test]
async fn crates_skips_dormant_optional_alternate_registry_edges() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    server
        .mock("GET", "/fo/re/foreign_optional_root")
        .with_status(200)
        .with_body(crate_line_with_features(
            "foreign_optional_root",
            "1.0.0",
            &sha256_hex(b"foreign-optional-root"),
            &dep("foreign-child", "^1", "normal", false),
            "{}",
            "{}",
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/fo/re/foreign-child")
        .with_status(200)
        .with_body(crate_line_with_features(
            "foreign-child",
            "1.0.0",
            &sha256_hex(b"foreign-child"),
            r#"{"name":"private-crate","req":"^1","features":[],"optional":true,"default_features":true,"target":null,"kind":"normal","package":null,"registry":"https://registry.example/index"}"#,
            "{}",
            "{}",
        ))
        .create_async()
        .await;

    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let entries = protocol
        .resolve_graph("foreign_optional_root", "1")
        .await
        .unwrap();
    assert_eq!(entries.len(), 2);
    let child = entries
        .iter()
        .find(|entry| entry.name == "foreign-child")
        .unwrap();
    assert!(
        child
            .extra_markers
            .iter()
            .any(|marker| marker == "optional:private-crate")
    );
}

#[tokio::test]
async fn crates_semver_selection_cases() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let bytes = b"SERDE_CRATE";
    let cksum = sha256_hex(bytes);
    let body = [
        crate_line("serde", "1.0.219", false, &cksum, ""),
        crate_line("serde", "2.0.0", false, &cksum, ""),
        crate_line("serde", "1.0.100", false, &cksum, ""),
    ]
    .join("\n");
    server
        .mock("GET", "/se/rd/serde")
        .with_status(200)
        .with_body(body)
        .create_async()
        .await;

    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());

    // ^1.0 → highest 1.x (2.0.0 excluded)
    assert_eq!(
        protocol.resolve("serde", "^1.0").await.unwrap().version,
        "1.0.219"
    );
    // =1.0.100 → exact
    assert_eq!(
        protocol.resolve("serde", "=1.0.100").await.unwrap().version,
        "1.0.100"
    );
    // >=1.0, <2.0 → 1.0.219
    assert_eq!(
        protocol
            .resolve("serde", ">=1.0, <2.0")
            .await
            .unwrap()
            .version,
        "1.0.219"
    );
    // ~1.0.100 → >=1.0.100 <1.1.0 → 1.0.219 (higher, same minor)
    assert_eq!(
        protocol.resolve("serde", "~1.0.100").await.unwrap().version,
        "1.0.219"
    );

    server
        .mock("GET", "/wi/nd/windows-sys")
        .with_status(200)
        .with_body(
            [
                crate_line("windows-sys", "0.61.0", false, &cksum, ""),
                crate_line("windows-sys", "0.61.1", false, &cksum, ""),
                crate_line("windows-sys", "0.61.2", false, &cksum, ""),
            ]
            .join("\n"),
        )
        .create_async()
        .await;
    assert_eq!(
        protocol
            .resolve("windows-sys", ">=0.48.0, <=0.61")
            .await
            .unwrap()
            .version,
        "0.61.2",
        "Cargo accepts semver-compatible minor comparator upper bounds"
    );
}

#[tokio::test]
async fn crates_resolve_download_verify_store_ref_full_path() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let bytes = b"SERDE_CRATE_ARCHIVE";
    let cksum = sha256_hex(bytes);

    let index_mock = server
        .mock("GET", "/se/rd/serde")
        .with_status(200)
        .with_body(crate_line("serde", "1.0.219", false, &cksum, ""))
        .create_async()
        .await;
    let download_mock = server
        .mock("GET", "/serde/1.0.219/download")
        .with_status(200)
        .with_body(bytes.as_slice())
        .create_async()
        .await;

    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let entry = protocol.resolve("serde", "^1.0").await.unwrap();
    let downloaded = protocol.download(&entry).await.unwrap();
    protocol.verify(&entry, &downloaded).unwrap();

    let expected_hex = blake3::hash(bytes).to_hex().to_string();
    let expected_ref = format!("files/blake3/{}/{}", &expected_hex[..2], expected_hex);
    assert_eq!(protocol.store_ref(&downloaded), expected_ref);

    index_mock.assert_async().await;
    download_mock.assert_async().await;
}

#[tokio::test]
async fn crates_sha256_mismatch_fails_closed() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let bytes = b"SERDE_CRATE_ARCHIVE";
    let wrong_cksum = sha256_hex(b"TAMPERED_BYTES");

    server
        .mock("GET", "/se/rd/serde")
        .with_status(200)
        .with_body(crate_line("serde", "1.0.219", false, &wrong_cksum, ""))
        .create_async()
        .await;
    server
        .mock("GET", "/serde/1.0.219/download")
        .with_status(200)
        .with_body(bytes.as_slice())
        .create_async()
        .await;

    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let entry = protocol.resolve("serde", "^1.0").await.unwrap();
    let downloaded = protocol.download(&entry).await.unwrap();
    let err = protocol.verify(&entry, &downloaded).unwrap_err();
    assert!(matches!(err, mgc_types::MgError::Integrity(_)), "{err:?}");
}
