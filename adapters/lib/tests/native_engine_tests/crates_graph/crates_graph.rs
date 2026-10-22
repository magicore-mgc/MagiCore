//! Verify Crates graph and v3 lock projection through the native Lib resolver.
//! Kiểm tra graph Crates và lock v3 qua resolver native của Lib.

use super::super::support::{crate_line, mock_server, rust_manifest};
use mgc_lib_adapter::native::engine::resolve_with_protocol;
use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::sha256_hex;
use mgc_resolver::protocols::{CratesProtocol, RegistryProtocol, ResolvedEntry};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

struct CrossRootConflictProtocol;
#[async_trait::async_trait]
impl RegistryProtocol for CrossRootConflictProtocol {
    async fn resolve(&self, name: &str, range: &str) -> mgc_types::MgResult<ResolvedEntry> {
        let (version, deps) = match name {
            "root-a" => ("1.0.0", vec![("shared".to_string(), "^1".to_string())]),
            "root-b" => ("1.0.0", vec![("shared".to_string(), "^2".to_string())]),
            "shared" if range == "^1" => ("1.5.0", vec![]),
            "shared" if range == "^2" => ("2.1.0", vec![]),
            _ => {
                return Err(mgc_types::MgError::Other(format!(
                    "unexpected {name}@{range}"
                )));
            }
        };
        Ok(ResolvedEntry {
            name: name.to_string(),
            version: version.to_string(),
            deps,
            artifact_url: format!("https://registry.invalid/{name}/{version}"),
            sha256: "00".repeat(32),
            extra_markers: vec![],
        })
    }

    async fn download(&self, _entry: &ResolvedEntry) -> mgc_types::MgResult<Vec<u8>> {
        Ok(vec![])
    }
}

pub(crate) async fn resolve_with_protocol_builds_graph_and_v3_lock_entries() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let serde_bytes = b"SERDE_CRATE";
    let core_bytes = b"SERDE_CORE";
    let serde_cksum = sha256_hex(serde_bytes);
    let core_cksum = sha256_hex(core_bytes);
    let base = server.url();

    server
        .mock("GET", "/se/rd/serde")
        .with_status(200)
        .with_body(crate_line(
            "serde",
            "1.0.219",
            &serde_cksum,
            r#"{"name":"serde_core","req":"1.0","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal","package":null}"#,
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/se/rd/serde_core")
        .with_status(200)
        .with_body(crate_line("serde_core", "1.0.5", &core_cksum, ""))
        .create_async()
        .await;

    let protocol = CratesProtocol::with_download_base(&base, &base);
    let resolution = resolve_with_protocol(
        &protocol,
        EcosystemTag::Rust,
        "crates://sparse+https://index.crates.io",
        &rust_manifest(),
    )
    .await
    .unwrap();

    // ResolvedGraph: serde + serde_core, correct versions and edges.
    // Graph có đúng hai package, version và cạnh phụ thuộc.
    assert_eq!(resolution.graph.packages.len(), 2);
    let serde = resolution
        .graph
        .packages
        .iter()
        .find(|p| p.id.name_str() == "serde")
        .unwrap();
    assert_eq!(serde.id.version().to_string(), "1.0.219");
    assert_eq!(serde.deps.len(), 1);
    assert_eq!(serde.deps[0].name_str(), "serde_core");
    assert_eq!(serde.deps[0].version().to_string(), "1.0.5");
    assert_eq!(serde.integrity, format!("sha256-{serde_cksum}"));

    // v3 lock entries: ecosystem/provenance/registry/artifact recorded.
    // Lock v3 ghi ecosystem, provenance, registry và artifact.
    assert_eq!(resolution.lock_packages.len(), 2);
    let lock_serde = resolution
        .lock_packages
        .iter()
        .find(|p| p.name == "serde")
        .unwrap();
    assert_eq!(lock_serde.ecosystem, EcosystemTag::Rust);
    assert_eq!(
        lock_serde.registry.as_deref(),
        Some("crates://sparse+https://index.crates.io")
    );
    let provenance = lock_serde.provenance.as_ref().unwrap();
    assert_eq!(provenance.source_kind, "native-resolve");
    let artifact = lock_serde.artifact.as_ref().unwrap();
    assert_eq!(artifact.url, format!("{base}/serde/1.0.219/download"));
    assert_eq!(
        artifact.downloaded_from,
        "crates://sparse+https://index.crates.io"
    );
    // blake3 content_hash is filled at install, not resolve — honest empty.
    // Hash chỉ có sau install nên resolve phải để rỗng.
    assert!(artifact.content_hash.is_empty());
    assert!(lock_serde.store_ref.is_none());
}

pub(crate) async fn native_engine_preserves_multiple_versions_and_exact_dependency_edges() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let dep = |name: &str, range: &str| {
        format!(
            r#"{{"name":"{name}","req":"{range}","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal","package":null}}"#
        )
    };
    server
        .mock("GET", "/ro/ot/root")
        .with_status(200)
        .with_body(crate_line(
            "root",
            "1.0.0",
            &sha256_hex(b"root"),
            &format!("{},{}", dep("left", "^1"), dep("right", "^1")),
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/le/ft/left")
        .with_status(200)
        .with_body(crate_line(
            "left",
            "1.0.0",
            &sha256_hex(b"left"),
            &dep("embassy-sync", "^0.6"),
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/ri/gh/right")
        .with_status(200)
        .with_body(crate_line(
            "right",
            "1.0.0",
            &sha256_hex(b"right"),
            &dep("embassy-sync", "^0.8"),
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/em/ba/embassy-sync")
        .with_status(200)
        .with_body(
            [
                crate_line("embassy-sync", "0.6.2", &sha256_hex(b"sync-0.6"), ""),
                crate_line("embassy-sync", "0.8.0", &sha256_hex(b"sync-0.8"), ""),
            ]
            .join("\n"),
        )
        .create_async()
        .await;

    let mut manifest = Manifest::new("demo-lib", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("root").unwrap(),
            VersionRange::parse("^1").unwrap(),
        ),
        false,
        false,
        false,
    );
    let resolution = resolve_with_protocol(
        &CratesProtocol::with_download_base(&server.url(), &server.url()),
        EcosystemTag::Rust,
        "crates://sparse+https://index.crates.io",
        &manifest,
    )
    .await
    .unwrap();

    let left = resolution
        .graph
        .packages
        .iter()
        .find(|package| package.id.name_str() == "left")
        .unwrap();
    let right = resolution
        .graph
        .packages
        .iter()
        .find(|package| package.id.name_str() == "right")
        .unwrap();
    assert_eq!(left.deps[0].to_string(), "embassy-sync@0.6.2");
    assert_eq!(right.deps[0].to_string(), "embassy-sync@0.8.0");
    assert_eq!(
        resolution
            .lock_packages
            .iter()
            .find(|package| package.name == "left")
            .unwrap()
            .dependencies,
        ["embassy-sync@0.6.2"]
    );
}

pub(crate) async fn resolve_with_protocol_rejects_cross_root_version_conflicts() {
    let mut manifest = Manifest::new("demo-lib", Ecosystem::Lib);
    for name in ["root-a", "root-b"] {
        manifest.add_dep(
            DependencySpec::new(PackageName::new(name).unwrap(), VersionRange::star()),
            false,
            false,
            false,
        );
    }

    let error = resolve_with_protocol(
        &CrossRootConflictProtocol,
        EcosystemTag::Rust,
        "test://registry",
        &manifest,
    )
    .await
    .unwrap_err();

    assert!(matches!(error, mgc_types::MgError::DependencyConflict(_)));
    assert!(error.to_string().contains("ambiguous lock graph"));
}
