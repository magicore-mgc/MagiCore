//! Integrity Projection source regression test.
//! Test hồi quy source cho integrity projection.

use base64::Engine as _;

use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::ResolvedEntry;

use crate::native::engine::engine::integrity_projection::resolved_integrity;

fn entry(sha256: &str, markers: Vec<String>) -> ResolvedEntry {
    ResolvedEntry {
        name: "demo".to_string(),
        version: "1.0.0".to_string(),
        deps: Vec::new(),
        artifact_url: "https://registry.invalid/demo".to_string(),
        sha256: sha256.to_string(),
        extra_markers: markers,
    }
}

#[test]
fn preserves_sha256_and_requires_one_64_byte_nuget_sha512_marker() {
    let rust = resolved_integrity(EcosystemTag::Rust, &entry("aabb", Vec::new())).unwrap();
    assert_eq!(rust, "sha256-aabb");

    let digest = base64::engine::general_purpose::STANDARD.encode([0_u8; 64]);
    let nuget = resolved_integrity(
        EcosystemTag::NuGet,
        &entry("", vec![format!("sha512:{digest}")]),
    )
    .unwrap();
    assert_eq!(nuget, format!("sha512-{digest}"));

    let malformed = base64::engine::general_purpose::STANDARD.encode([0_u8; 63]);
    assert!(
        resolved_integrity(
            EcosystemTag::NuGet,
            &entry("", vec![format!("sha512:{malformed}")]),
        )
        .is_err()
    );
}
