//! Resolve requested packages and update the in-memory manifest delta.
//! Resolve package được yêu cầu và cập nhật manifest delta trong bộ nhớ.

use super::super::types::AddProgress;
use super::super::*;

/// Resolve packages in request order while the caller holds the mutation lock.
/// Resolve theo thứ tự request trong lúc caller giữ mutation lock.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn resolve_requested_packages(
    adapter: &dyn PackageAdapter,
    root: &Path,
    packages: Vec<String>,
    version: Option<String>,
    dev: bool,
    exact: bool,
    optional: bool,
    peer: bool,
    no_save: bool,
    global: bool,
    group: &str,
    manifest_after_add: &mut Option<Manifest>,
) -> Result<AddProgress> {
    let mut added_ids = Vec::new();
    let mut added_packages = Vec::new();
    let mut changed_any = false;
    for package in packages {
        let spec = mgc_types::DependencySpec::parse(&package)?;
        let name = spec.name;
        let range = if let Some(v) = version.as_ref() {
            Some(mgc_types::VersionRange::parse(v)?)
        } else if spec.range.is_star() {
            None
        } else {
            Some(spec.range)
        };

        let spinner = create_spinner(&format!("  Resolving {}...", package));
        let opts = AddOptions {
            dev,
            optional,
            peer,
            exact,
            no_save,
            global,
        };
        let add_started_at = std::time::Instant::now();
        let PreparedAdd {
            id: pkg_id,
            range: saved_range,
        } = adapter
            .prepare_add(root, &name, range.as_ref(), opts)
            .await?;
        profile_install_mark("adapter_prepare_add", add_started_at);
        spinner.finish_and_clear();
        let requested_range = range
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "*".to_string());
        let resolved_version = pkg_id.version().to_string();

        if !no_save {
            if let Some(manifest) = manifest_after_add.as_mut() {
                let mut saved_spec = DependencySpec::new(name.clone(), saved_range.clone());
                saved_spec.dev = dev;
                saved_spec.optional = optional;
                saved_spec.peer = peer;
                let changed = manifest.add_dep(saved_spec, dev, optional, peer);
                if changed {
                    changed_any = true;
                    added_ids.push(pkg_id.clone());
                    added_packages.push(AddedPackage {
                        id: pkg_id.clone(),
                        dev,
                        optional,
                        peer,
                    });
                    if resolved_version == "0.0.0" {
                        mgc_ui::info(&format!(
                            "  {}@{} saved to {}",
                            pkg_id.name_str(),
                            requested_range,
                            group
                        ));
                    } else {
                        mgc_ui::info(&format!(
                            "  {}@{} added to {}",
                            pkg_id.name_str(),
                            resolved_version,
                            group
                        ));
                    }
                    mgc_ui::success(&format!("Added {}", package));
                } else {
                    mgc_ui::info(&format!(
                        "  {} already present in {}, skipping",
                        pkg_id.name_str(),
                        group
                    ));
                }
            }
        } else {
            mgc_ui::info(&format!(
                "  {}@{} checked (--no-save, manifest unchanged)",
                pkg_id.name_str(),
                requested_range
            ));
        }
    }

    Ok(AddProgress {
        added_ids,
        added_packages,
        changed_any,
    })
}
