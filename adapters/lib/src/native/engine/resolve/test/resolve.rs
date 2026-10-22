//! Resolve source regression test.
//! Test hồi quy source cho resolve.

use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::{RegistryProtocol, RegistryRoot, ResolvedEntry};
use mgc_types::{
    DependencySpec, Ecosystem, Manifest, MgError, MgResult, PackageName, VersionRange,
};

use crate::native::engine::engine::resolve::{
    resolve_with_protocol, resolve_with_protocol_including_dev,
};

struct StaticProtocol(Vec<ResolvedEntry>);

#[async_trait::async_trait]
impl RegistryProtocol for StaticProtocol {
    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry> {
        Err(MgError::Other(format!("unexpected resolve {name}@{range}")))
    }

    async fn download(&self, _entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        Ok(Vec::new())
    }

    async fn resolve_graph_roots_with_options(
        &self,
        roots: &[RegistryRoot],
    ) -> MgResult<Vec<ResolvedEntry>> {
        Ok(self
            .0
            .iter()
            .filter(|entry| roots.iter().any(|root| root.name == entry.name))
            .cloned()
            .collect())
    }
}

fn entry(name: &str) -> ResolvedEntry {
    ResolvedEntry {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        deps: Vec::new(),
        artifact_url: format!("https://registry.invalid/{name}/1.0.0"),
        sha256: String::new(),
        extra_markers: Vec::new(),
    }
}

fn manifest_with_runtime_and_dev() -> Manifest {
    let mut manifest = Manifest::new("demo-lib", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("runtime-tool").unwrap(),
            VersionRange::parse("^1.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("test-tool").unwrap(),
            VersionRange::parse("^1.0").unwrap(),
        ),
        true,
        false,
        false,
    );
    manifest
}

#[tokio::test]
async fn wrappers_keep_runtime_only_and_including_dev_contracts() {
    let protocol = StaticProtocol(vec![entry("runtime-tool"), entry("test-tool")]);
    let manifest = manifest_with_runtime_and_dev();

    let runtime = resolve_with_protocol(&protocol, EcosystemTag::Rust, "crates://test", &manifest)
        .await
        .unwrap();
    assert_eq!(runtime.graph.packages.len(), 1);
    assert_eq!(runtime.graph.packages[0].id.name_str(), "runtime-tool");
    assert!(!runtime.graph.packages[0].dev);

    let all = resolve_with_protocol_including_dev(
        &protocol,
        EcosystemTag::Rust,
        "crates://test",
        &manifest,
    )
    .await
    .unwrap();
    let dev = all
        .graph
        .packages
        .iter()
        .find(|package| package.id.name_str() == "test-tool")
        .unwrap();
    assert!(dev.dev);
}
