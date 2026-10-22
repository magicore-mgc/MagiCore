//! Resolve a real Flutter package version and stage its lock entries.
//! Resolve phiên bản package Flutter thật và chuẩn bị entry lock.

use crate::adapter::AppAdapter;
use crate::language::AppLanguage;
use mgc_lib_adapter::native::engine::resolve_with_protocol;
use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::PubProtocol;
use mgc_types::adapter::{AddOptions, PreparedAdd};
use mgc_types::{
    DependencySpec, Ecosystem, Manifest, MgError, MgResult, PackageId, PackageName, VersionRange,
};
use std::path::Path;

pub(in crate::adapter::package_adapter) async fn prepare_add(
    adapter: &AppAdapter,
    project_root: &Path,
    name: &PackageName,
    range: Option<&VersionRange>,
    opts: AddOptions,
) -> MgResult<PreparedAdd> {
    adapter.pending_install.clear()?;
    let AppLanguage::Flutter = adapter.language else {
        return Err(MgError::Other(format!(
            "native prepare-add is flutter-only; {:?} stays toolchain-owned",
            adapter.language
        )));
    };
    let _ = project_root;
    // Resolve the real version first because pubspec cannot persist `*`.
    // Resolve phiên bản thật trước vì pubspec không lưu được range `*`.
    let wanted = range.cloned().unwrap_or_else(VersionRange::star);
    let mut scratch = Manifest::new("scratch", Ecosystem::App);
    scratch.add_dep(
        DependencySpec::new(name.clone(), wanted.clone()),
        opts.dev,
        opts.optional,
        opts.peer,
    );
    let protocol = PubProtocol::from_env();
    let resolution =
        resolve_with_protocol(&protocol, EcosystemTag::Dart, "pub://pub.dev", &scratch).await?;
    let resolved = resolution
        .graph
        .packages
        .iter()
        .find(|p| p.id.name_str() == name.as_str())
        .ok_or_else(|| {
            MgError::Other(format!(
                "native resolve returned no entry for '{}' — refusing to book an unresolved dep",
                name.as_str()
            ))
        })?;
    let pinned = if wanted.is_star() {
        VersionRange::parse(&resolved.id.version().to_string())?
    } else {
        wanted
    };
    adapter
        .pending_install
        .store_lock(resolution.lock_packages)?;
    Ok(PreparedAdd {
        id: PackageId::new(name.clone(), resolved.id.version().clone()),
        range: pinned,
    })
}
