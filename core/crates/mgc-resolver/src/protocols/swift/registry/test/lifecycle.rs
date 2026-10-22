//! Resolve, download, verify, and materialize lifecycle case. / Case lifecycle resolve, download, verify và materialize.

use super::support::{LIB_PACKAGE_SWIFT, build_archive, mock_server};
use mgc_resolver::protocols::sha256_hex;
use mgc_resolver::protocols::{RegistryProtocol, SwiftRegistryProtocol};

#[tokio::test]
async fn swift_registry_roundtrip_resolve_verify_materialize() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let base = server.url();
    let archive = build_archive(&[
        ("Package.swift", LIB_PACKAGE_SWIFT.as_bytes()),
        (
            "Sources/Lib/Lib.swift",
            b"export func hello() {}".as_slice(),
        ),
    ]);
    let sha = sha256_hex(&archive);

    server
        .mock("GET", "/scope/lib/scope.json")
        .with_status(200)
        .with_body(r#"{"versions":["1.0.0","1.2.0","2.0.0"]}"#)
        .create_async()
        .await;
    // Highest matching wins (SwiftPM semantics): from:1.0.0 + 2.0.0 → 2.0.0.
    // (Cao nhất khớp thắng (ngữ nghĩa SwiftPM): from:1.0.0 + 2.0.0 → 2.0.0.)
    server
        .mock("GET", "/scope/lib/1.2.0.sha256")
        .with_status(200)
        .with_body(format!("{sha}\n"))
        .create_async()
        .await;
    let zip_mock = server
        .mock("GET", "/scope/lib/1.2.0.zip")
        .with_status(200)
        .with_body(archive.as_slice())
        .create_async()
        .await;

    let protocol = SwiftRegistryProtocol::with_registry(&base);
    let entry = protocol
        .resolve("scope/lib", "from:1.0.0")
        .await
        .expect("test operation must succeed");
    // `from:1.0.0` is upToNextMajor (>=1.0.0, <2.0.0) → the highest
    // matching is 1.2.0, never 2.0.0.
    // (from:1.0.0 là upToNextMajor (>=1.0.0, <2.0.0) → cao nhất khớp là
    // 1.2.0, không bao giờ 2.0.0.)
    assert_eq!(entry.version, "1.2.0");
    assert_eq!(entry.sha256, sha);
    assert!(
        entry
            .extra_markers
            .iter()
            .any(|m| m == "swift-registry:scope"),
        "{:?}",
        entry.extra_markers
    );
    // Transitive deps from the archive's Package.swift.
    // (Dep bắc cầu từ Package.swift trong archive.)
    assert_eq!(
        entry.deps,
        vec![
            ("other/lib".to_string(), "from:2.0.0".to_string()),
            (
                "github.com/example/gitdep".to_string(),
                "exact:1.0.0".to_string()
            ),
        ]
    );

    // download() serves the resolve-cached bytes (single fetch).
    // (download() phục vụ byte đã cache lúc resolve (một lần tải).)
    let downloaded = protocol
        .download(&entry)
        .await
        .expect("test operation must succeed");
    protocol
        .verify(&entry, &downloaded)
        .expect("test operation must succeed");
    zip_mock.assert_async().await;

    // Checksum mismatch fails closed.
    // (Checksum lệch fail-closed.)
    let mut tampered = entry.clone();
    tampered.sha256 = "0".repeat(64);
    let err = protocol
        .verify(&tampered, &downloaded)
        .expect_err("test operation must fail");
    assert!(matches!(err, mgc_types::MgError::Integrity(_)), "{err:?}");

    // Checkouts materialization: {root}/checkouts/scope.lib-1.2.0/.
    // (Materialize checkouts: {root}/checkouts/scope.lib-1.2.0/.)
    let root = tempfile::tempdir().expect("test operation must succeed");
    let checkout = protocol
        .materialize(&entry, &downloaded, root.path())
        .expect("test operation must succeed");
    assert_eq!(
        checkout,
        root.path().join("checkouts").join("scope.lib-1.2.0")
    );
    assert!(checkout.join("Package.swift").is_file());
    assert!(checkout.join("Sources/Lib/Lib.swift").is_file());
}
