//! Explain who requires a package from Web lockfile graphs.
//! Giải thích package được yêu cầu bởi dependency nào trong lockfile Web.

use anyhow::Result;
use mgc_types::adapter::PackageAdapter;
use std::path::Path;

/// Show reverse dependencies for a package in v2/v3/v4 lockfiles.
/// Hiển thị phụ thuộc ngược của package trong lockfile v2/v3/v4.
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
            let roots = payload
                .root_dependencies
                .iter()
                .map(|edge| edge_name(edge))
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
            let roots = lock
                .root_dependencies
                .iter()
                .map(|edge| edge_name(edge))
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

/// Remove version/range suffixes while preserving a scoped package name.
/// Bỏ hậu tố version/range nhưng giữ nguyên tên package có scope.
pub(in crate::commands::core::web::dependency_why) fn edge_name(edge: &str) -> String {
    let edge = edge.trim();
    if let Some(rest) = edge.strip_prefix('@') {
        match rest.find('@') {
            Some(index) => format!("@{}", &rest[..index]),
            None => edge.to_string(),
        }
    } else {
        match edge.find('@') {
            Some(index) => edge[..index].to_string(),
            None => edge.to_string(),
        }
    }
}
