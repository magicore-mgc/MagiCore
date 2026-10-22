//! Resolve update targets serially and apply changed versions to memory.
//! Resolve target tuần tự và cập nhật phiên bản đổi vào manifest trong bộ nhớ.

use super::super::types::UpdateTarget;
use super::super::*;

/// Preserve adapter call order, current-version skips and dependency flags.
/// Giữ thứ tự gọi adapter, nhánh bỏ qua version hiện tại và cờ dependency.
pub(crate) async fn resolve_update_targets(
    adapter: &dyn PackageAdapter,
    root: &Path,
    manifest: &mut Manifest,
    targets: &[UpdateTarget],
) -> Result<Vec<mgc_types::adapter::UpdatedPackage>> {
    let mut updated = Vec::new();
    for target in targets {
        let package_name = PackageName::new(&target.name)?;
        let spinner = create_spinner(&format!("  Updating {}...", target.name));
        let prepared = adapter
            .prepare_add(
                root,
                &package_name,
                None,
                AddOptions {
                    dev: target.dev,
                    optional: target.optional,
                    peer: target.peer,
                    ..Default::default()
                },
            )
            .await?;
        spinner.finish_and_clear();
        let resolved_version = prepared.id.version().to_string();
        if resolved_version == target.current_version {
            info(&format!(
                "  {} already latest ({})",
                target.name, target.current_version
            ));
            continue;
        }
        let mut spec = DependencySpec::new(package_name, prepared.range);
        spec.dev = target.dev;
        spec.optional = target.optional;
        spec.peer = target.peer;
        manifest.add_dep(spec, target.dev, target.optional, target.peer);
        updated.push(mgc_types::adapter::UpdatedPackage {
            name: target.name.clone(),
            from_version: target.current_version.clone(),
            to_version: resolved_version,
        });
    }
    Ok(updated)
}
