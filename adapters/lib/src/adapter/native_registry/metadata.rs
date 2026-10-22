//! Registry metadata is selected by lane and operation.
//! Metadata registry được chọn theo lane và thao tác.

use crate::adapter::types::ResolveFirst;
use mgc_lockfile::EcosystemTag;

/// The operation selects registry identity when a lane persists lock rows.
/// Thao tác chọn registry identity khi lane ghi các row lock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::adapter) enum NativeRegistryOperation {
    ProjectResolve,
    SinglePackageAdd,
}

/// Lock metadata shared by project resolution and single-package add.
/// Metadata lock dùng chung giữa resolve dự án và add package đơn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct NativeRegistryMetadata {
    pub(super) ecosystem: EcosystemTag,
    pub(super) registry: &'static str,
}

/// Return the unchanged ecosystem and registry identity for a lane operation.
/// Trả ecosystem và registry identity hiện tại theo thao tác của lane.
pub(super) fn metadata_for_lane(
    lane: &ResolveFirst,
    operation: NativeRegistryOperation,
) -> NativeRegistryMetadata {
    match (lane, operation) {
        (ResolveFirst::Rust, NativeRegistryOperation::ProjectResolve) => NativeRegistryMetadata {
            ecosystem: EcosystemTag::Rust,
            registry: "crates://sparse+https://index.crates.io",
        },
        (ResolveFirst::Rust, NativeRegistryOperation::SinglePackageAdd) => NativeRegistryMetadata {
            ecosystem: EcosystemTag::Rust,
            registry: "crates://crates.io",
        },
        (ResolveFirst::Python, _) => NativeRegistryMetadata {
            ecosystem: EcosystemTag::Python,
            registry: "pypi://pypi.org",
        },
        (ResolveFirst::Go, _) => NativeRegistryMetadata {
            ecosystem: EcosystemTag::Go,
            registry: "go://proxy.golang.org",
        },
        (ResolveFirst::DotNet, _) => NativeRegistryMetadata {
            ecosystem: EcosystemTag::NuGet,
            registry: "nuget://api.nuget.org",
        },
        (ResolveFirst::JavaPom, _) => NativeRegistryMetadata {
            ecosystem: EcosystemTag::Maven,
            registry: "maven://repo.maven.apache.org",
        },
    }
}
