//! Crates protocol engine tests — hermetic via mockito (no real network).
//! Test engine crates — hermetic qua mockito (không mạng thật).

#![allow(clippy::unwrap_used)]

use mgc_resolver::protocols::sha256_hex;
use mgc_resolver::protocols::{CratesProtocol, RegistryProtocol};

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
async fn crates_resolves_complete_lock_closure_with_optional_and_target_edges() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let serde_bytes = b"SERDE_CRATE_1.0.219";
    let core_bytes = b"SERDE_CORE_1.0.5";
    let windows_bytes = b"WINDOWS_SYS_0.59.0";
    let serde_cksum = sha256_hex(serde_bytes);
    let core_cksum = sha256_hex(core_bytes);
    let windows_cksum = sha256_hex(windows_bytes);

    // serde 1.0.219 → serde_core + dormant optional serde_derive + Windows-only
    // windows-sys. The optional edge is marked but not fetched; the target edge
    // remains in the lock closure for Cargo builds on other operating systems.
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

    assert_eq!(entries.len(), 3, "target-qualified lock edges are resolved");
    let serde = &entries[0];
    assert_eq!(serde.name, "serde");
    assert_eq!(serde.version, "1.0.219");
    assert_eq!(
        serde.deps,
        vec![
            ("serde_core".to_string(), "1.0".to_string()),
            ("windows-sys".to_string(), "^0.59".to_string()),
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

    let core = &entries[1];
    assert_eq!(core.name, "serde_core");
    assert_eq!(core.version, "1.0.5");
    assert_eq!(entries[2].name, "windows-sys");
    assert!(
        serde
            .extra_markers
            .iter()
            .any(|marker| marker == "target:cfg(windows)")
    );
}

#[tokio::test]
async fn crates_feature_resolution_enables_selected_optional_edges_only() {
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
                dep("indexmap", "^2", "normal", true),
            ),
            r#"{"default":["derive"],"derive":["dep:quote"],"proc-macro":["proc-macro2/proc-macro"],"weak-forward":["proc-macro2?/span"]}"#,
            "{}",
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
        !by_name.contains_key("indexmap"),
        "dormant optional dependency must stay unresolved"
    );
    assert!(by_name["syn"].deps.iter().any(|(name, _)| name == "quote"));
    assert!(
        by_name["syn"]
            .extra_markers
            .iter()
            .any(|marker| marker == "optional:indexmap")
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
