//! Explain who requires a package from Web lockfile graphs.
//! Giải thích package được yêu cầu bởi dependency nào trong lockfile Web.

use anyhow::Result;
use mgc_types::adapter::PackageAdapter;
use std::path::Path;

/// Show reverse dependencies for a package in v2/v3/v4 lockfiles.
/// Hiển thị phụ thuộc ngược của package trong lockfile v2/v3/v4.
#[path = "edge_name/mod.rs"]
mod edge_name;

/// Re-export so module tests and the parent can keep reaching this helper.
pub(in crate::commands::core::web::dependency_why) use edge_name::edge_name;

pub async fn why(adapter: &dyn PackageAdapter, root: &Path, package: &str) -> Result<()> {
    if adapter.name() != "web" {
        return Err(super::error::WhyError::unsupported_core().into());
    }
    let lock_path = root.join("mgc.lock");
    // Reverse lookup reads the same dependency edges the installer wrote.
    // Tra cứu ngược dùng đúng dependency edge do installer ghi.
    let lock_bytes = match mgc_lockfile::read_lockfile_bytes(&lock_path) {
        Ok(bytes) => bytes,
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            return Err(super::error::WhyError::lock_missing().into());
        }
        Err(error) => return Err(error.into()),
    };
    let text = String::from_utf8(lock_bytes)
        .map_err(|error| anyhow::anyhow!("mgc.lock is not valid UTF-8: {error}"))?;
    let version = toml::from_str::<toml::Value>(&text)
        .ok()
        .and_then(|value| value.get("version")?.as_str().map(str::to_string))
        .unwrap_or_default();
    type WhyNode = (String, String, Vec<String>);
    let (nodes, roots): (Vec<WhyNode>, Vec<String>) = match version.as_str() {
        version if mgc_lockfile::v4::is_v4_schema_version(version) => {
            let doc = mgc_lockfile::canonical::parse_v4_document(&text)
                .map_err(|error| anyhow::anyhow!("parse mgc.lock v4 failed: {error}"))?;
            let report = mgc_lockfile::policy::verify_v4_math(&doc)?;
            mgc_lockfile::policy::enforce_policy(
                &report,
                mgc_lockfile::policy::LockPolicyMode::Warn,
                &[],
            )?;
            if !report.signed {
                mgc_ui::warning(
                    "mgc.lock v4 digest is valid, but the lock is unsigned; signer trust was not evaluated",
                );
            } else {
                mgc_ui::warning(
                    "mgc.lock v4 signature math is valid; signer trust was not evaluated by this command",
                );
            }
            let payload = doc.payload();
            let nodes = payload
                .packages
                .iter()
                .map(|package| {
                    (
                        package.key.name.clone(),
                        package.key.version.clone(),
                        package
                            .edges
                            .iter()
                            .map(|edge| edge.target_key.name.clone())
                            .collect(),
                    )
                })
                .collect();
            // Same prefix problem as the legacy branch below: a v4 root pin is
            // `mgc-root-v1:<ecosystem>:<name>@<version>`, so `edge_name` alone
            // yields `mgc-root-v1:web:nanoid`.
            // (Cùng vấn đề prefix như nhánh legacy: root pin v4 là
            // `mgc-root-v1:<eco>:<name>@<version>`.)
            let roots = payload
                .root_dependencies
                .iter()
                .map(|edge| edge_name(mgc_lockfile::root_pin::parse_root_pin(edge).package_id))
                .collect();
            (nodes, roots)
        }
        _ => {
            let lock = mgc_lockfile::parser::parse_lockfile(&text)
                .map_err(|error| anyhow::anyhow!("parse mgc.lock failed: {error}"))?;
            let nodes = lock
                .packages
                .iter()
                .map(|package| {
                    (
                        package.name.clone(),
                        package.version.clone(),
                        package
                            .dependencies
                            .iter()
                            .map(|edge| edge_name(edge))
                            .collect(),
                    )
                })
                .collect();
            // Root pins carry the `mgc-root-v1:<ecosystem>:` prefix, so running
            // `edge_name` on them produced `mgc-root-v1:web:nanoid` instead of
            // `nanoid`. Every direct dependency then looked like an orphan and
            // `mgc why` told the user to prune it. `parse_root_pin` already exists
            // for exactly this encoding.
            // (Root pin có prefix `mgc-root-v1:<ecosystem>:` nên `edge_name` trả
            // `mgc-root-v1:web:nanoid` thay vì `nanoid`. Mọi dependency trực tiếp
            // trông như mồ côi và `mgc why` khuyên prune nó. `parse_root_pin` đã có
            // sẵn cho đúng encoding này.)
            // Read BOTH root fields. v3 locks write
            // `[root_dependencies_by_owner]` and leave the flat
            // `root_dependencies` vector empty, so reading only the flat field made
            // every direct dependency look like an orphan — `mgc why nanoid` told
            // the user to prune a dependency declared in `package.json`.
            // (Đọc CẢ HAI field: lock v3 ghi `root_dependencies_by_owner` và để
            // vector phẳng rỗng, nên đọc mỗi vector khiến mọi dependency trực tiếp
            // trông như mồ côi.)
            let roots: Vec<String> = lock
                .root_dependencies
                .iter()
                .chain(
                    lock.root_dependencies_by_owner
                        .values()
                        .flat_map(|owned| owned.iter()),
                )
                .map(|edge| edge_name(mgc_lockfile::root_pin::parse_root_pin(edge).package_id))
                .collect();
            (nodes, roots)
        }
    };

    let target = edge_name(package);
    let target_node = nodes.iter().find(|(name, _, _)| name == &target);
    let Some((_, target_version, _)) = target_node else {
        return Err(super::error::WhyError::package_not_pinned(package).into());
    };
    let mut dependents: Vec<&WhyNode> = nodes
        .iter()
        .filter(|(name, _, dependencies)| {
            name.as_str() != target.as_str()
                && dependencies.iter().any(|dependency| dependency == &target)
        })
        .collect();
    dependents.sort_by(|left, right| left.0.cmp(&right.0));
    if dependents.is_empty() && !roots.iter().any(|root_name| root_name == &target) {
        mgc_ui::info(&format!(
            "{target}@{target_version} is locked but nothing in mgc.lock depends on it (likely a leftover pin — `mgc dedupe`/`prune` may drop it)."
        ));
        return Ok(());
    }
    let mut lines = vec![format!("{target}@{target_version} is required by:")];
    for (name, version, _) in dependents.iter().take(20) {
        lines.push(format!("  {name}@{version}"));
    }
    if dependents.len() > 20 {
        lines.push(format!("  ... and {} more", dependents.len() - 20));
    }
    if roots.iter().any(|root_name| root_name == &target) {
        lines.push("(project root — direct dependency)".to_string());
    }
    for line in &lines {
        mgc_ui::info(line);
    }
    Ok(())
}
