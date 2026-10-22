//! Fail-closed registry cases. / Các case registry fail-closed.

use super::support::mock_server;
use mgc_resolver::protocols::{RegistryProtocol, SwiftRegistryProtocol};

#[tokio::test]
async fn swift_registry_missing_checksum_fails_closed() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let base = server.url();
    server
        .mock("GET", "/scope/lib/scope.json")
        .with_status(200)
        .with_body(r#"{"versions":["1.0.0"]}"#)
        .create_async()
        .await;
    // No .sha256 endpoint (404) — a registry without checksums is never
    // trusted.
    // (Không endpoint .sha256 (404) — registry không checksum không bao giờ
    // được tin.)
    let protocol = SwiftRegistryProtocol::with_registry(&base);
    let err = protocol
        .resolve("scope/lib", "from:1.0.0")
        .await
        .expect_err("test operation must fail");
    assert!(matches!(err, mgc_types::MgError::Integrity(_)), "{err:?}");
}

#[tokio::test]
async fn swift_registry_without_scope_fails_closed() {
    let protocol = SwiftRegistryProtocol::with_registry("http://127.0.0.1:9");
    let err = protocol
        .resolve("lib", "from:1.0.0")
        .await
        .expect_err("test operation must fail");
    assert!(
        err.to_string().contains("scope/name"),
        "guidance must name the scope requirement: {err}"
    );
}

#[tokio::test]
async fn swift_file_dependency_fails_closed_without_git_process() {
    let protocol = SwiftRegistryProtocol::with_registry("http://127.0.0.1:9");

    let err = protocol
        .resolve("file:///tmp/not-a-real-repository", "tag:1.0.0")
        .await
        .expect_err("test operation must fail");
    assert!(
        err.to_string().contains("blocked"),
        "file:// must fail closed naming the block: {err}"
    );
}

#[tokio::test]
async fn swift_git_dependency_is_unsupported_even_with_old_opt_in_env() {
    let result = SwiftRegistryProtocol::with_registry("http://127.0.0.1:9")
        .resolve("127.0.0.1/x.git", "branch:main")
        .await;
    let err = result.expect_err("test operation must fail");
    assert!(
        err.to_string().contains("refuses to spawn Git"),
        "the legacy opt-in must not enable an external Git process: {err}"
    );
}
