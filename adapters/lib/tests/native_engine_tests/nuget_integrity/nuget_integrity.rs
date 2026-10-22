//! Verify NuGet marker validation rejects missing, malformed, and duplicate digests.
//! Kiểm tra marker NuGet từ chối digest thiếu, sai dạng và trùng lặp.

use mgc_lib_adapter::native::engine::resolve_with_protocol;
use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::{RegistryProtocol, ResolvedEntry};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

struct MarkerProtocol(Vec<String>);
#[async_trait::async_trait]
impl RegistryProtocol for MarkerProtocol {
    async fn resolve(&self, name: &str, _range: &str) -> mgc_types::MgResult<ResolvedEntry> {
        Ok(ResolvedEntry {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            deps: Vec::new(),
            artifact_url: format!("https://registry.invalid/{name}/1.0.0"),
            sha256: String::new(),
            extra_markers: self.0.clone(),
        })
    }

    async fn download(&self, _entry: &ResolvedEntry) -> mgc_types::MgResult<Vec<u8>> {
        Ok(Vec::new())
    }
}

pub(crate) async fn nuget_resolution_rejects_missing_malformed_or_ambiguous_sha512_markers() {
    let valid_digest = {
        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD.encode([0_u8; 64])
    };
    let cases = [
        Vec::new(),
        vec!["sha512:YQ==".to_string()],
        vec![
            format!("sha512:{valid_digest}"),
            format!("sha512:{valid_digest}"),
        ],
    ];
    let mut manifest = Manifest::new("demo", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(PackageName::new("demo.lib").unwrap(), VersionRange::star()),
        false,
        false,
        false,
    );

    for markers in cases {
        let error = resolve_with_protocol(
            &MarkerProtocol(markers),
            EcosystemTag::NuGet,
            "nuget://local-test",
            &manifest,
        )
        .await
        .unwrap_err();
        assert!(matches!(error, mgc_types::MgError::Integrity(_)));
    }
}
