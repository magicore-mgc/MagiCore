//! Verify the Lib Rust adapter uses the configured native Crates engine.
//! Kiểm tra adapter Rust của Lib dùng native Crates engine đã cấu hình.

use super::super::support::environment_override::{
    EnvironmentOverrideGuard, lock_environment_access,
};
use super::super::support::{crate_line, mock_server, rust_manifest};
use mgc_resolver::protocols::sha256_hex;
use mgc_types::DependencyResolver;

pub(crate) async fn lib_adapter_resolve_uses_native_engine_via_env() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let cksum = sha256_hex(b"SERDE_CRATE");
    server
        .mock("GET", "/se/rd/serde")
        .with_status(200)
        .with_body(crate_line("serde", "1.0.219", &cksum, ""))
        .create_async()
        .await;

    let _environment_lock = lock_environment_access();
    let _environment = EnvironmentOverrideGuard::set(
        &_environment_lock,
        &[
            ("MGC_CRATES_INDEX_URL", server.url()),
            ("MGC_CRATES_DOWNLOAD_URL", server.url()),
        ],
    );

    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(
        tmp.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    let adapter = mgc_lib_adapter::adapter_for(tmp.path(), None, None)
        .unwrap()
        .unwrap();
    let graph = adapter.resolve(&rust_manifest()).await.unwrap();

    assert_eq!(graph.packages.len(), 1);
    assert_eq!(graph.packages[0].id.name_str(), "serde");
    assert_eq!(graph.packages[0].id.version().to_string(), "1.0.219");
}
