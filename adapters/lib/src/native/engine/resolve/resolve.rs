//! Resolve owner.
//! Phụ trách resolve của native engine.

use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::RegistryProtocol;
use mgc_types::{Manifest, MgResult};

use crate::native::engine::engine::package_projection::NativeResolution;

use crate::native::engine::engine::dependency_closure::runtime_reachable;
use crate::native::engine::engine::entry_index::index_entries;
use crate::native::engine::engine::package_projection::project_packages;
use crate::native::engine::engine::root_selection::{
    registry_roots, resolve_root_ids, selected_roots,
};

/// Resolve a manifest's regular dependencies through a native protocol engine.
/// Resolve dependencies thường của manifest qua một engine protocol native.
pub async fn resolve_with_protocol(
    protocol: &dyn RegistryProtocol,
    ecosystem: EcosystemTag,
    registry: &str,
    manifest: &Manifest,
) -> MgResult<NativeResolution> {
    resolve_with_protocol_inner(protocol, ecosystem, registry, manifest, false).await
}

/// Resolve both runtime and development dependencies through a native protocol.
/// Resolve cả dependency runtime và development qua protocol native.
///
/// Native Python and App/Flutter callers need dev packages in the installed
/// graph so test/build tools stay in the MGC lock and do not resolve separately.
/// Other callers keep the runtime-only contract unless they select this method.
/// (Python native và App/Flutter cần graph dev để test/build tools cùng nằm
/// trong lock MGC, không tự resolve ngoài.)
pub async fn resolve_with_protocol_including_dev(
    protocol: &dyn RegistryProtocol,
    ecosystem: EcosystemTag,
    registry: &str,
    manifest: &Manifest,
) -> MgResult<NativeResolution> {
    resolve_with_protocol_inner(protocol, ecosystem, registry, manifest, true).await
}

async fn resolve_with_protocol_inner(
    protocol: &dyn RegistryProtocol,
    ecosystem: EcosystemTag,
    registry: &str,
    manifest: &Manifest,
    include_dev_dependencies: bool,
) -> MgResult<NativeResolution> {
    let roots = selected_roots(manifest, include_dev_dependencies);
    let root_specs = registry_roots(&roots);
    let entries = protocol
        .resolve_graph_roots_with_options(&root_specs)
        .await?;
    let (chosen, by_name) = index_entries(&entries)?;

    // A shared transitive package remains runtime if any runtime root reaches it.
    // (Package transitive dùng chung vẫn là runtime nếu có root runtime chạm tới.)
    let direct_ids = resolve_root_ids(protocol, &roots, &entries, &by_name)?;
    let runtime_roots = resolve_root_ids(
        protocol,
        &selected_roots(manifest, false),
        &entries,
        &by_name,
    )?;
    let runtime_reachable = runtime_reachable(runtime_roots, &chosen, &by_name)?;

    project_packages(
        ecosystem,
        registry,
        &entries,
        &chosen,
        &by_name,
        &direct_ids,
        &runtime_reachable,
    )
}
