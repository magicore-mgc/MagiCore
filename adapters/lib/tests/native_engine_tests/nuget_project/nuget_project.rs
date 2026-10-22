//! Verify native NuGet project resolution through its local mock registry.
//! Kiểm tra resolve project NuGet qua registry mock cục bộ.

use super::super::support::environment_override::{
    EnvironmentOverrideGuard, lock_environment_access,
};
use super::super::support::mock_server;
use mgc_types::{DependencyResolver, PackageAdapter};

pub(crate) async fn lib_adapter_csproj_project_resolves_natively_via_env() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let base = server.url();
    // Hand-assembled store-method nupkg (no zip crate).
    // (Nupkg method store tự ghép — không crate zip.)
    let nupkg = {
        let name = "Demo.Lib.nuspec";
        let data = "<package><metadata><id>Demo.Lib</id><dependencies /></metadata></package>"
            .to_string()
            .into_bytes();
        let mut out: Vec<u8> = Vec::new();
        let crc = mgc_resolver::protocols::zip_reader::crc32(&data);
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
        out.extend_from_slice(&data);
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
    let sha512_b64 = {
        use base64::Engine as _;
        use sha2::Digest as _;
        base64::engine::general_purpose::STANDARD.encode(sha2::Sha512::digest(&nupkg))
    };

    server
        .mock("GET", "/v3/index.json")
        .with_status(200)
        .with_body(format!(
            r#"{{"resources":[{{"@id":"{base}/reg/","@type":"RegistrationsBaseUrl/3.6.0"}},{{"@id":"{base}/flat/","@type":"PackageBaseAddress/3.0.0"}}]}}"#
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/flat/demo.lib/index.json")
        .with_status(200)
        .with_body(r#"{"versions":["1.2.3"]}"#)
        .create_async()
        .await;
    server
        .mock("GET", "/reg/demo.lib/index.json")
        .with_status(200)
        // Live nuget.org registration leaves OMIT packageHash — the
        // resolver must follow catalogEntry @id to the catalog document.
        // (Registration thật không có packageHash — resolver phải theo @id.)
        .with_body(format!(
            r#"{{"items":[{{"items":[{{"catalogEntry":{{"@id":"{base}/catalog/demo.lib.1.2.3.json","id":"Demo.Lib","version":"1.2.3","listed":true}}}}]}}]}}"#
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/catalog/demo.lib.1.2.3.json")
        .with_status(200)
        .with_body(format!(
            "{{\"packageHash\":\"{sha512_b64}\",\"packageHashAlgorithm\":\"SHA512\"}}"
        ))
        .create_async()
        .await;
    server
        .mock("GET", "/flat/demo.lib/1.2.3/demo.lib.nuspec")
        .with_status(200)
        .with_body(r#"<package><metadata><dependencies /></metadata></package>"#)
        .create_async()
        .await;

    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(
        tmp.path().join("Demo.App.csproj"),
        r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><PackageReference Include="Demo.Lib" Version="1.2.3" /></ItemGroup></Project>"#,
    )
    .unwrap();

    let _environment_lock = lock_environment_access();
    let _environment = EnvironmentOverrideGuard::set(
        &_environment_lock,
        &[("MGC_NUGET_INDEX_URL", format!("{base}/v3/index.json"))],
    );
    let adapter = mgc_lib_adapter::adapter_for(tmp.path(), None, None)
        .unwrap()
        .unwrap();
    let manifest = adapter.parse_manifest(tmp.path()).await.unwrap();
    assert_eq!(manifest.name, "Demo.App");
    let graph = adapter.resolve(&manifest).await.unwrap();
    drop(_environment);
    assert_eq!(graph.packages.len(), 1);
    assert_eq!(graph.packages[0].id.name_str(), "Demo.Lib");
    assert_eq!(graph.packages[0].id.version().to_string(), "1.2.3");
    assert_eq!(graph.packages[0].integrity, format!("sha512-{sha512_b64}"));
}
