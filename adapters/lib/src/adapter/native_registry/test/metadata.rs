//! Lock the per-lane ecosystem and registry identity contract.
//! Cố định hợp đồng ecosystem và registry identity theo lane.

use super::super::metadata::{NativeRegistryOperation, metadata_for_lane};
use crate::adapter::types::ResolveFirst;
use mgc_lockfile::EcosystemTag;

#[test]
fn operation_metadata_preserves_each_native_lane_identity() {
    let rust_resolve =
        metadata_for_lane(&ResolveFirst::Rust, NativeRegistryOperation::ProjectResolve);
    let rust_add = metadata_for_lane(
        &ResolveFirst::Rust,
        NativeRegistryOperation::SinglePackageAdd,
    );
    assert_eq!(rust_resolve.ecosystem, EcosystemTag::Rust);
    assert_eq!(
        rust_resolve.registry,
        "crates://sparse+https://index.crates.io"
    );
    assert_eq!(rust_add.ecosystem, EcosystemTag::Rust);
    assert_eq!(rust_add.registry, "crates://crates.io");

    for (lane, ecosystem, registry) in [
        (
            ResolveFirst::Python,
            EcosystemTag::Python,
            "pypi://pypi.org",
        ),
        (ResolveFirst::Go, EcosystemTag::Go, "go://proxy.golang.org"),
        (
            ResolveFirst::DotNet,
            EcosystemTag::NuGet,
            "nuget://api.nuget.org",
        ),
        (
            ResolveFirst::JavaPom,
            EcosystemTag::Maven,
            "maven://repo.maven.apache.org",
        ),
    ] {
        for operation in [
            NativeRegistryOperation::ProjectResolve,
            NativeRegistryOperation::SinglePackageAdd,
        ] {
            let metadata = metadata_for_lane(&lane, operation);
            assert_eq!(metadata.ecosystem, ecosystem);
            assert_eq!(metadata.registry, registry);
        }
    }
}
