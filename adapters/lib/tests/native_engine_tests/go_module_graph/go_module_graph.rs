//! Verify Go proxy resolution and v3 lock projection through the Lib native engine.
//! Kiểm tra resolve Go proxy và lock v3 qua native engine Lib.

use super::super::support::mock_server;
use mgc_lib_adapter::native::engine::resolve_with_protocol;
use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::sha256_hex;
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

pub(crate) async fn resolve_with_protocol_builds_go_module_graph_and_v3_lock_entries() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let base = server.url();

    // Zip for the module + its proxy artifacts.
    // (Zip cho module + các artifact proxy.)
    let zip_bytes = {
        // Minimal single-entry store zip, hand-assembled (no zip crate).
        // (Zip store một entry tối giản, ghép thủ công — không crate zip.)
        let name = "example.com/lib@v1.4.0/go.mod";
        let data = b"module example.com/lib\n";
        let mut out: Vec<u8> = Vec::new();
        let crc = mgc_resolver::protocols::zip_reader::crc32(data);
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(data);
        let cd_offset = out.len() as u32;
        let mut cd = Vec::new();
        cd.extend_from_slice(b"PK\x01\x02");
        cd.extend_from_slice(&20u16.to_le_bytes());
        cd.extend_from_slice(&20u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u32.to_le_bytes());
        cd.extend_from_slice(&crc.to_le_bytes());
        cd.extend_from_slice(&(data.len() as u32).to_le_bytes());
        cd.extend_from_slice(&(data.len() as u32).to_le_bytes());
        cd.extend_from_slice(&(name.len() as u16).to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u32.to_le_bytes());
        cd.extend_from_slice(&0u32.to_le_bytes());
        cd.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&cd);
        let cd_size = out.len() as u32 - cd_offset;
        out.extend_from_slice(b"PK\x05\x06");
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&cd_size.to_le_bytes());
        out.extend_from_slice(&cd_offset.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out
    };
    let zip_hash = sha256_hex(&zip_bytes);

    server
        .mock("GET", "/example.com/lib/@v/list")
        .with_status(200)
        .with_body("v1.4.0\n")
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/lib/@v/v1.4.0.info")
        .with_status(200)
        .with_body(r#"{"Version":"v1.4.0","Time":"2024-03-04T05:06:07Z"}"#)
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/lib/@v/v1.4.0.mod")
        .with_status(200)
        .with_body("module example.com/lib\n")
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/lib/@v/v1.4.0.ziphash")
        .with_status(200)
        .with_body(zip_hash.as_str())
        .create_async()
        .await;

    let protocol = mgc_resolver::protocols::GoModProtocol::with_sum_base(&base, &base);
    let mut manifest = Manifest::new("example.com/app", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("example.com/lib").unwrap(),
            VersionRange::parse("1.4.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let resolution = resolve_with_protocol(
        &protocol,
        EcosystemTag::Go,
        "go://proxy.golang.org",
        &manifest,
    )
    .await
    .unwrap();

    assert_eq!(resolution.graph.packages.len(), 1);
    let pkg = &resolution.graph.packages[0];
    assert_eq!(pkg.id.name_str(), "example.com/lib");
    assert_eq!(pkg.id.version().to_string(), "1.4.0");

    assert_eq!(resolution.lock_packages.len(), 1);
    let lock = &resolution.lock_packages[0];
    assert_eq!(lock.ecosystem, EcosystemTag::Go);
    assert_eq!(lock.registry.as_deref(), Some("go://proxy.golang.org"));
    assert_eq!(lock.integrity, format!("sha256-{zip_hash}"));
    assert_eq!(
        lock.provenance.as_ref().unwrap().source_kind,
        "native-resolve"
    );
    assert_eq!(
        lock.artifact.as_ref().unwrap().url,
        format!("{base}/example.com/lib/@v/v1.4.0.zip")
    );
}
