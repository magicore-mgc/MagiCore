//! Manifest and lock update after audit-fix resolution succeeds.
//! Cập nhật manifest và lock sau khi resolve audit-fix thành công.

use mgc_types::{DependencySpec, MgError, MgResult, PackageId, VersionRange};
use std::path::Path;

use crate::lockfile::write_web_lockfile_with_state;
use crate::manifest::{parse_manifest, write_manifest};

pub async fn run_audit_fix<F, Fut>(
    project_root: &Path,
    vulnerable: &[PackageId],
    resolve_fn: F,
) -> MgResult<usize>
where
    F: FnOnce(mgc_types::Manifest) -> Fut,
    Fut: std::future::Future<Output = MgResult<mgc_types::adapter::ResolvedGraph>>,
{
    if vulnerable.is_empty() {
        return Ok(0);
    }

    let mut manifest = parse_manifest(project_root)?;
    let names: std::collections::HashSet<&str> =
        vulnerable.iter().map(|id| id.name_str()).collect();

    let flags = [
        (false, false, false),
        (true, false, false),
        (false, false, true),
        (false, true, false),
    ];
    let mut bumps: Vec<(DependencySpec, bool, bool, bool)> = Vec::new();
    for (group, (dev, optional, peer)) in manifest.dep_groups().iter().zip(flags) {
        for spec in group.1 {
            if names.contains(spec.name.as_str()) {
                bumps.push((spec.clone(), dev, optional, peer));
            }
        }
    }
    if bumps.is_empty() {
        return Err(MgError::Other(
            "audit --fix: found no matching dependency to bump (validate manifest)".into(),
        ));
    }

    for (spec, dev, optional, peer) in &bumps {
        let new_spec = DependencySpec::new(spec.name.clone(), VersionRange::star());
        manifest.remove_dep(spec.name.as_str());
        manifest.add_dep(new_spec, *dev, *optional, *peer);
    }

    let graph = resolve_fn(manifest.clone()).await?;
    write_manifest(project_root, &manifest)?;
    write_web_lockfile_with_state(project_root, &graph, "fixed")?;
    Ok(bumps.len())
}
