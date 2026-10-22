//! Owns Go requirement projection and exact Web-owned lock comparison, including existing replace policy. — Sở hữu projection requirement Go và đối chiếu chính xác lock thuộc Web, gồm chính sách replace hiện có.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;

use super::super::lock::locked_root_versions;

/// Declared Go module requirements from `go.mod` (full module paths).
/// Requirement module Go từ `go.mod` (đường dẫn module đầy đủ).
///
/// Reserved for the first Go lane that grows a dev flow; no caller yet
/// (follow-up, not dead design).
/// (Dành cho lane Go đầu tiên có dev flow; chưa có caller
/// (việc tiếp theo, không phải thiết kế chết).)
#[allow(dead_code)]
pub fn declared_go_deps(go_mod: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_require = false;
    for line in go_mod.lines() {
        let line = line.trim();
        if line.starts_with("require (") {
            in_require = true;
            continue;
        }
        if in_require && line == ")" {
            in_require = false;
            continue;
        }
        let requirement = if in_require {
            Some(line)
        } else {
            line.strip_prefix("require ").map(str::trim)
        };
        // Quoted module paths are legal go.mod — strip the quotes.
        // (Đường dẫn module có quote là go.mod hợp lệ — bỏ quote.)
        if let Some(req) = requirement
            && let Some(module) = req
                .trim_end_matches("// indirect")
                .split_whitespace()
                .next()
                .map(|m| m.trim_matches('"'))
            && !module.is_empty()
            && !module.starts_with("//")
        {
            names.push(module.to_string());
        }
    }
    names.sort();
    names.dedup();
    names
}
/// Require exact Go requirement pins to match the Web-owned lock before build.
/// Yêu cầu pin Go chính xác khớp lock thuộc Web trước khi build.
pub fn check_web_go_build_freshness(project_root: &Path) -> Result<()> {
    let path = project_root.join("go.mod");
    let Some(content) = mgc_config::project::read_regular_project_text(&path, "go.mod")? else {
        return Ok(());
    };
    let parsed = mgc_resolver::protocols::go::parse_go_mod(&content)
        .map_err(|error| crate::error::freshness_manifest_invalid("go.mod", &error.to_string()))?;

    let mut declared = BTreeMap::<String, String>::new();
    for requirement in parsed.requires {
        if parsed
            .excludes
            .iter()
            .any(|(module, version)| module == &requirement.path && version == &requirement.version)
        {
            continue;
        }
        declared.insert(
            requirement.path,
            requirement
                .version
                .strip_prefix('v')
                .unwrap_or(&requirement.version)
                .to_string(),
        );
    }

    if !parsed.replaces.is_empty() && !declared.is_empty() {
        return Err(crate::error::go_build_freshness_unverified(
            "go.mod replace directives do not have a verified lock projection",
        ));
    }

    let lock_path = project_root.join("mgc.lock");
    if !lock_path.exists() {
        if declared.is_empty() {
            return Ok(());
        }
        return Err(crate::error::stale_lock_needs_install(
            "mgc.lock is missing for Go requirements",
        ));
    }

    let owner = crate::commands::core::shared::lock_owner_core(project_root, "web")?;
    let locked = locked_root_versions(project_root, &owner, mgc_lockfile::EcosystemTag::Go)?;
    if declared.len() != locked.len() {
        return Err(crate::error::stale_lock_needs_install(
            "Go requirement roots differ from the Web-owned lock",
        ));
    }
    for (module, version) in declared {
        let versions = locked.get(&module).map(Vec::as_slice).unwrap_or_default();
        let matches_exact_pin =
            versions.len() == 1 && versions[0].strip_prefix('v').unwrap_or(&versions[0]) == version;
        if !matches_exact_pin {
            return Err(crate::error::stale_lock_needs_install(&format!(
                "Go module '{module}' does not have the exact declared version '{version}' in the Web-owned lock"
            )));
        }
    }
    Ok(())
}
