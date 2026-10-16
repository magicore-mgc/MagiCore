//! Cross-ecosystem freshness projections — projection freshness đa ecosystem.
//!
//! Pre-flight dependency consistency across supported ecosystems (Spec rev4
//! §8/§15 task 7): each projection extracts declared dependency names and
//! compares them with owner/ecosystem-scoped direct roots in the lock. Only
//! dependency metadata participates — script-only edits never trigger a
//! resolve.
//! (Nhất quán dependency pre-flight qua các ecosystem được hỗ trợ: mỗi
//! projection trích tên dependency khai báo và so với root trực tiếp trong
//! lock theo owner/ecosystem. Chỉ metadata dependency tham gia — sửa
//! scripts không trigger resolve.)

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Context, Result};

/// Read package identities from either supported lock schema.
/// Đọc danh tính package từ cả hai schema lock được hỗ trợ.
fn locked_packages(project_root: &Path) -> Result<Vec<(String, String)>> {
    let lock_path = project_root.join("mgc.lock");
    let document = mgc_lockfile::parser::load_lock_document(&lock_path).map_err(|e| {
        crate::error::stale_lock_needs_install(&format!("cannot read mgc.lock: {e}"))
    })?;
    Ok(match document {
        mgc_lockfile::LockDocument::Legacy(lockfile) => lockfile
            .packages
            .into_iter()
            .map(|package| (package.name, package.version))
            .collect(),
        mgc_lockfile::LockDocument::V4(lockfile) => lockfile
            .packages
            .into_iter()
            .map(|package| (package.key.name, package.key.version))
            .collect(),
    })
}

/// Dev entries require a valid lock even when the manifest has no direct
/// dependencies. Test pre-flight keeps its documented lockless fixture
/// exception in [`check_any_freshness`].
/// Dev luôn cần lock hợp lệ, kể cả khi manifest rỗng dependency.
/// Return direct root names for one owner/ecosystem slice without confusing
/// transitive packages with dependencies declared by the project.
/// Trả tên root trực tiếp theo owner/ecosystem, không nhầm package bắc cầu
/// thành dependency do project khai báo.
fn locked_root_versions(
    project_root: &Path,
    owner: &str,
    ecosystem: mgc_lockfile::EcosystemTag,
) -> Result<BTreeMap<String, Vec<String>>> {
    use mgc_lockfile::{EcosystemTag, LockDocument, parse_root_pin};

    let lock_path = project_root.join("mgc.lock");
    let document = mgc_lockfile::parser::load_lock_document(&lock_path).map_err(|e| {
        crate::error::stale_lock_needs_install(&format!("cannot read mgc.lock: {e}"))
    })?;

    let mut versions = BTreeMap::<String, Vec<String>>::new();
    match document {
        LockDocument::Legacy(lockfile) => {
            let owner_roots: Vec<&String> = lockfile
                .root_dependencies_by_owner
                .get(owner)
                .into_iter()
                .flatten()
                .collect();
            let legacy_web_roots = owner == "web"
                && lockfile.version == "2"
                && ecosystem == EcosystemTag::Web
                && owner_roots.is_empty();
            let roots: Vec<&String> = if legacy_web_roots {
                lockfile.root_dependencies.iter().collect()
            } else {
                owner_roots
            };

            for root in roots {
                let parsed = parse_root_pin(root);
                if parsed.ecosystem.is_some_and(|tag| tag != ecosystem) {
                    continue;
                }
                let Some((name, version)) = parsed.package_id.rsplit_once('@') else {
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "invalid direct root pin '{root}'"
                    )));
                };
                if name.is_empty() || version.is_empty() {
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "invalid direct root pin '{root}'"
                    )));
                }
                let candidates = lockfile
                    .packages
                    .iter()
                    .filter(|package| package.name == name && package.version == version)
                    .filter(|package| {
                        package.ecosystem == ecosystem
                            || (legacy_web_roots
                                && ecosystem == EcosystemTag::Web
                                && package.ecosystem == EcosystemTag::Other)
                    })
                    .filter(|package| {
                        package
                            .owner_core
                            .as_deref()
                            .is_none_or(|value| value == owner)
                    })
                    .count();
                if candidates != 1 {
                    let reason = if candidates == 0 {
                        "does not identify a locked package"
                    } else {
                        "identifies multiple locked packages"
                    };
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "direct root pin '{root}' {reason}"
                    )));
                }
                versions
                    .entry(name.to_string())
                    .or_default()
                    .push(version.to_string());
            }
        }
        LockDocument::V4(lockfile) => {
            let roots = lockfile
                .root_dependencies_by_owner
                .get(owner)
                .into_iter()
                .flatten();
            for root in roots {
                let parsed = parse_root_pin(root);
                if parsed.ecosystem.is_some_and(|tag| tag != ecosystem) {
                    continue;
                }
                let Some((name, version)) = parsed.package_id.rsplit_once('@') else {
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "invalid direct root pin '{root}'"
                    )));
                };
                if name.is_empty() || version.is_empty() {
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "invalid direct root pin '{root}'"
                    )));
                }
                let candidates = lockfile
                    .packages
                    .iter()
                    .filter(|package| {
                        package.key.name == name
                            && package.key.version == version
                            && package.key.ecosystem == ecosystem
                    })
                    .count();
                if candidates != 1 {
                    let reason = if candidates == 0 {
                        "does not identify a locked package"
                    } else {
                        "identifies multiple locked packages"
                    };
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "direct root pin '{root}' {reason}"
                    )));
                }
                versions
                    .entry(name.to_string())
                    .or_default()
                    .push(version.to_string());
            }
        }
    }
    for package_versions in versions.values_mut() {
        package_versions.sort();
        package_versions.dedup();
    }
    Ok(versions)
}

/// Require declared dependency names and direct lock roots to be identical.
/// Yêu cầu tập tên khai báo trùng khớp root trực tiếp trong lock.
fn check_declared_roots(declared: &[String], locked_roots: &[String]) -> Result<()> {
    let declared: BTreeSet<&str> = declared.iter().map(String::as_str).collect();
    let locked: BTreeSet<&str> = locked_roots.iter().map(String::as_str).collect();
    let missing: Vec<&str> = declared.difference(&locked).copied().collect();
    let stale: Vec<&str> = locked.difference(&declared).copied().collect();
    if !missing.is_empty() || !stale.is_empty() {
        let mut differences = Vec::new();
        if !missing.is_empty() {
            differences.push(format!("not root-locked: {}", missing.join(", ")));
        }
        if !stale.is_empty() {
            differences.push(format!("no longer declared: {}", stale.join(", ")));
        }
        return Err(crate::error::stale_lock_needs_install(
            &differences.join("; "),
        ));
    }
    Ok(())
}

fn check_manifest_roots(
    project_root: &Path,
    owner_fallback: &str,
    ecosystem: mgc_lockfile::EcosystemTag,
    declared: &[String],
) -> Result<()> {
    let owner = crate::commands::core::shared::lock_owner_core(project_root, owner_fallback)?;
    let locked = locked_root_versions(project_root, &owner, ecosystem)?;
    let locked_names = locked.keys().cloned().collect::<Vec<_>>();
    check_declared_roots(declared, &locked_names)
}

/// Web dev freshness includes both direct-root identity and supported npm
/// semver constraints. Non-semver references retain root-presence validation
/// because their source identity is not represented as a version range.
/// (Freshness web dev so cả root trực tiếp lẫn range semver npm đã hỗ trợ.
/// Reference không phải semver chỉ kiểm tra root vì lock chưa biểu diễn source.)
pub fn check_dev_npm_freshness_against_lock(
    project_root: &Path,
    declared: &BTreeMap<String, Vec<String>>,
) -> Result<()> {
    let owner = crate::commands::core::shared::lock_owner_core(project_root, "web")?;
    let locked = locked_root_versions(project_root, &owner, mgc_lockfile::EcosystemTag::Web)?;
    let declared_names = declared.keys().cloned().collect::<Vec<_>>();
    let locked_names = locked.keys().cloned().collect::<Vec<_>>();
    check_declared_roots(&declared_names, &locked_names)?;

    for (name, raw_ranges) in declared {
        let parsed_ranges = raw_ranges
            .iter()
            .filter_map(|raw_range| {
                let range = raw_range.strip_prefix("workspace:").unwrap_or(raw_range);
                npm_range_is_semver(range).then_some((raw_range, range))
            })
            .map(|(raw_range, range)| {
                mgc_types::VersionRange::parse(range)
                    .map(|parsed| (raw_range, parsed))
                    .map_err(|error| {
                        crate::error::freshness_manifest_invalid(
                            "package.json",
                            &format!(
                                "dependency '{name}' has invalid version range '{raw_range}': {error}"
                            ),
                        )
                    })
            })
            .collect::<Result<Vec<_>>>()?;
        if parsed_ranges.is_empty() {
            continue;
        }
        let versions = locked.get(name).map(Vec::as_slice).unwrap_or_default();
        // Duplicate declarations across package sections constrain one
        // resolved direct root; one locked version must satisfy every
        // semver range, rather than letting separate versions satisfy each.
        // (Khai báo trùng tên ở nhiều section cùng ràng buộc một root; một
        // version trong lock phải thỏa mọi range semver.)
        let satisfies = versions.iter().any(|locked_version| {
            mgc_types::Version::parse(locked_version).is_ok_and(|version| {
                parsed_ranges
                    .iter()
                    .all(|(_, range)| range.matches(&version))
            })
        });
        if !satisfies {
            let locked_versions = if versions.is_empty() {
                "none".to_string()
            } else {
                versions.join(", ")
            };
            let declared_ranges = raw_ranges.join(" and ");
            return Err(crate::error::stale_lock_needs_install(&format!(
                "'{name}' declares '{declared_ranges}', but locked version(s) are {locked_versions}"
            )));
        }
    }
    Ok(())
}

fn npm_range_is_semver(range: &str) -> bool {
    let range = range.trim();
    if range.is_empty() || range == "*" || range.eq_ignore_ascii_case("x") {
        return true;
    }
    if range
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        return true;
    }
    if let Some(rest) = range.strip_prefix('v').or_else(|| range.strip_prefix('V'))
        && rest
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_digit())
    {
        return true;
    }
    matches!(
        range.chars().next(),
        Some('^' | '~' | '>' | '<' | '=' | '|')
    )
}

/// PEP 508 requirement to distribution name (`Foo_Bar>=1.0; marker` → `foo-bar`).
/// Requirement PEP 508 thành tên distribution.
fn pep508_name(requirement: &str) -> String {
    let bare = requirement.split(';').next().unwrap_or(requirement).trim();
    let end = bare
        .find(|c: char| " \t<>=!~([".contains(c))
        .unwrap_or(bare.len());
    bare[..end].trim().replace('_', "-").to_ascii_lowercase()
}

/// Declared Python runtime and MGC-managed dev dependencies from `pyproject.toml`.
/// Dependency runtime và nhóm dev do MGC quản lý trong `pyproject.toml`.
pub fn declared_python_deps(pyproject_toml: &str) -> Vec<String> {
    let Ok(table) = pyproject_toml.parse::<toml::Table>() else {
        return Vec::new();
    };
    let mut names = Vec::new();
    // Unselected optional extras are not roots in the default install graph.
    // Extra chưa được chọn không thuộc root graph của install mặc định.
    if let Some(deps) = table
        .get("project")
        .and_then(|v| v.get("dependencies"))
        .and_then(|v| v.as_array())
    {
        names.extend(
            deps.iter()
                .filter_map(|v| v.as_str())
                .map(pep508_name)
                .filter(|n| !n.is_empty()),
        );
    }
    if let Some(deps) = table
        .get("dependency-groups")
        .and_then(|v| v.get("dev"))
        .and_then(|v| v.as_array())
    {
        names.extend(
            deps.iter()
                .filter_map(|v| v.as_str())
                .map(pep508_name)
                .filter(|n| !n.is_empty()),
        );
    }
    names.sort();
    names.dedup();
    names
}

/// Declared Flutter dependencies from `pubspec.yaml` top-level mapping.
/// Dependency Flutter khai báo từ mapping top-level `pubspec.yaml`.
// Keep the public projection API for library consumers and non-app builds.
// Giữ API projection cho library; binary app dùng manifest đã mở rộng từ SDK.
#[cfg_attr(feature = "app", allow(dead_code))]
pub fn declared_flutter_deps(pubspec_yaml: &str) -> Vec<String> {
    let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(pubspec_yaml) else {
        return Vec::new();
    };
    let mut names = Vec::new();
    for section in ["dependencies", "dev_dependencies"] {
        if let Some(map) = value.get(section).and_then(|v| v.as_mapping()) {
            names.extend(map.keys().filter_map(|k| k.as_str()).map(|s| s.to_string()));
        }
    }
    names.sort();
    names.dedup();
    names
}

/// Declared Rust dependencies from `Cargo.toml` tables.
/// Dependency Rust khai báo từ các bảng `Cargo.toml`.
pub fn declared_cargo_deps(cargo_toml: &str) -> Vec<String> {
    let Ok(table) = cargo_toml.parse::<toml::Table>() else {
        return Vec::new();
    };
    let mut names = Vec::new();
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(deps) = table.get(section).and_then(|v| v.as_table()) {
            names.extend(deps.keys().cloned());
        }
    }
    // Target-specific tables (`[target.'cfg(...)'.dependencies]`).
    // (Bảng theo target (`[target.'cfg(...)'.dependencies]`).)
    if let Some(target) = table.get("target").and_then(|v| v.as_table()) {
        for triple in target.values().filter_map(|v| v.as_table()) {
            for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if let Some(deps) = triple.get(section).and_then(|v| v.as_table()) {
                    names.extend(deps.keys().cloned());
                }
            }
        }
    }
    names.sort();
    names.dedup();
    names
}

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

/// Read a manifest file, project its dependency names, and check the lock.
/// Returns `Ok(false)` when the manifest is absent (nothing to compare).
/// (Đọc manifest, projection tên dependency, rồi check lock. Trả `Ok(false)`
/// khi manifest vắng (không có gì để so).)
fn validate_manifest_syntax(file: &str, content: &str) -> Result<()> {
    match file {
        "package.json" => {
            let value = serde_json::from_str::<serde_json::Value>(content)
                .context("package.json is not valid JSON")?;
            let Some(root) = value.as_object() else {
                return Err(crate::error::freshness_manifest_invalid(
                    file,
                    "must contain a JSON object at the root",
                ));
            };
            for section in [
                "dependencies",
                "devDependencies",
                "peerDependencies",
                "optionalDependencies",
            ] {
                if root.get(section).is_some_and(|value| !value.is_object()) {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        &format!("field '{section}' must be an object"),
                    ));
                }
            }
        }
        "pyproject.toml" => {
            let value = toml::from_str::<toml::Value>(content)
                .with_context(|| format!("{file} is not valid TOML"))?;
            if let Some(project) = value.get("project") {
                let Some(project) = project.as_table() else {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        "field 'project' must be a table",
                    ));
                };
                if let Some(dependencies) = project.get("dependencies")
                    && dependencies
                        .as_array()
                        .is_none_or(|items| items.iter().any(|item| item.as_str().is_none()))
                {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        "field 'project.dependencies' must be an array of strings",
                    ));
                }
                if let Some(groups) = project.get("optional-dependencies") {
                    let Some(groups) = groups.as_table() else {
                        return Err(crate::error::freshness_manifest_invalid(
                            file,
                            "field 'project.optional-dependencies' must be a table",
                        ));
                    };
                    if groups.values().any(|items| {
                        items
                            .as_array()
                            .is_none_or(|items| items.iter().any(|item| item.as_str().is_none()))
                    }) {
                        return Err(crate::error::freshness_manifest_invalid(
                            file,
                            "optional dependency groups must be arrays of strings",
                        ));
                    }
                }
            }
        }
        "Cargo.toml" => {
            let value = toml::from_str::<toml::Value>(content)
                .with_context(|| format!("{file} is not valid TOML"))?;
            for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if value.get(section).is_some_and(|value| !value.is_table()) {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        &format!("field '{section}' must be a table"),
                    ));
                }
            }
        }
        "pubspec.yaml" => {
            let value = serde_yaml::from_str::<serde_yaml::Value>(content)
                .context("pubspec.yaml is not valid YAML")?;
            if !value.is_mapping() {
                return Err(crate::error::freshness_manifest_invalid(
                    file,
                    "must contain a YAML mapping at the root",
                ));
            }
            for section in ["dependencies", "dev_dependencies"] {
                if value.get(section).is_some_and(|value| !value.is_mapping()) {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        &format!("field '{section}' must be a mapping"),
                    ));
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn check_manifest_file(
    project_root: &Path,
    file: &str,
    require_lock: bool,
    owner_fallback: &str,
    ecosystem: mgc_lockfile::EcosystemTag,
    project: impl FnOnce(&str) -> Result<Vec<String>>,
) -> Result<bool> {
    let path = project_root.join(file);
    if !path.exists() {
        return Ok(false);
    }
    let has_lock = project_root.join("mgc.lock").exists();
    if require_lock && !has_lock {
        return Err(crate::error::stale_lock_needs_install(
            "mgc.lock is missing",
        ));
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    validate_manifest_syntax(file, &content)?;
    let declared = project(&content)?;
    if has_lock {
        check_manifest_roots(project_root, owner_fallback, ecosystem, &declared)?;
    }
    Ok(true)
}

/// Python freshness for AI/dev flows (`pyproject.toml`).
/// Freshness Python cho luồng AI/dev (`pyproject.toml`).
pub fn check_python_freshness(project_root: &Path) -> Result<()> {
    check_manifest_file(
        project_root,
        "pyproject.toml",
        true,
        "ai",
        mgc_lockfile::EcosystemTag::Python,
        |content| Ok(declared_python_deps(content)),
    )?;
    Ok(())
}

/// Flutter freshness for app/dev flows (`pubspec.yaml`).
/// Freshness Flutter cho luồng app/dev (`pubspec.yaml`).
pub fn check_flutter_freshness(project_root: &Path) -> Result<()> {
    check_flutter_manifest_freshness(project_root, true)?;
    Ok(())
}

/// Compare the same SDK-expanded registry roots used by native install.
/// So cùng root registry đã mở rộng từ SDK, không coi package SDK là pin Pub.
fn check_flutter_manifest_freshness(project_root: &Path, require_lock: bool) -> Result<bool> {
    #[cfg(not(feature = "app"))]
    {
        check_manifest_file(
            project_root,
            "pubspec.yaml",
            require_lock,
            "app",
            mgc_lockfile::EcosystemTag::Dart,
            |content| Ok(declared_flutter_deps(content)),
        )
    }
    #[cfg(feature = "app")]
    {
        use mgc_types::adapter::PackageAdapter;

        let Some(content) = mgc_config::project::read_regular_project_text(
            &project_root.join("pubspec.yaml"),
            "pubspec.yaml",
        )?
        else {
            return Ok(false);
        };
        let has_lock = project_root.join("mgc.lock").exists();
        if require_lock && !has_lock {
            return Err(crate::error::stale_lock_needs_install(
                "mgc.lock is missing",
            ));
        }
        validate_manifest_syntax("pubspec.yaml", &content)?;
        if !has_lock {
            return Ok(true);
        }
        let manifest = mgc_app_adapter::manifest::flutter::parse_pubspec(project_root)?;
        let adapter = mgc_app_adapter::AppAdapter::new(mgc_app_adapter::AppLanguage::Flutter);
        let effective = adapter.lock_replay_manifest(project_root, &manifest)?;
        let owner = crate::commands::core::shared::lock_owner_core(project_root, "app")?;
        let locked = locked_root_versions(project_root, &owner, mgc_lockfile::EcosystemTag::Dart)?;
        let mut declared = BTreeMap::<String, Vec<&mgc_types::VersionRange>>::new();
        for dependency in effective.all_dependencies() {
            declared
                .entry(dependency.name.to_string())
                .or_default()
                .push(&dependency.range);
        }
        let names = declared.keys().cloned().collect::<Vec<_>>();
        check_declared_roots(&names, &locked.keys().cloned().collect::<Vec<_>>())?;
        for (name, ranges) in declared {
            let versions = locked.get(&name).map(Vec::as_slice).unwrap_or_default();
            // One direct version must satisfy every project and SDK constraint.
            // Một version trực tiếp phải thỏa mọi constraint của project và SDK.
            let satisfies = versions.iter().any(|version| {
                mgc_types::Version::parse(version)
                    .is_ok_and(|version| ranges.iter().all(|range| range.matches(&version)))
            });
            if !satisfies {
                let requirements = ranges
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" and ");
                return Err(crate::error::stale_lock_needs_install(&format!(
                    "'{name}' declares '{requirements}', but locked version(s) are {}",
                    versions.join(", ")
                )));
            }
        }
        Ok(true)
    }
}

/// Cargo freshness for Rust dev flows (`Cargo.toml`, e.g. Bevy/ESP32).
/// Freshness Cargo cho luồng dev Rust (`Cargo.toml`, vd Bevy/ESP32).
pub fn check_cargo_freshness(project_root: &Path) -> Result<()> {
    check_manifest_file(
        project_root,
        "Cargo.toml",
        true,
        "game",
        mgc_lockfile::EcosystemTag::Rust,
        |content| Ok(declared_cargo_deps(content)),
    )?;
    Ok(())
}

/// Cargo freshness for Rust projects owned by the library core.
/// Freshness Cargo cho project Rust thuộc core library.
pub fn check_lib_cargo_freshness(project_root: &Path) -> Result<()> {
    check_manifest_file(
        project_root,
        "Cargo.toml",
        true,
        "lib",
        mgc_lockfile::EcosystemTag::Rust,
        |content| Ok(declared_cargo_deps(content)),
    )?;
    Ok(())
}

/// Known framework package → profile framework id.
/// Package framework đã biết → id framework cho profile.
fn framework_id_for_package(name: &str) -> Option<&'static str> {
    match name {
        "next" => Some("next"),
        "nuxt" => Some("nuxt"),
        "@angular/core" => Some("angular"),
        "@remix-run/react" | "@remix-run/node" | "@react-router/dev" => Some("remix"),
        "astro" => Some("astro"),
        "@sveltejs/kit" => Some("sveltekit"),
        "vite" => Some("vite"),
        "react-scripts" => Some("react-scripts"),
        _ => None,
    }
}

/// Resolve the web framework and its locked version for profile lookup
/// (Spec rev4 §6/§11): framework id from manifest dependencies, version
/// from the locked entry. `None` when no known framework or no lock entry.
/// (Xác định framework web và version đã lock để tra profile (Spec rev4
/// §6/§11): id framework từ dependency manifest, version từ entry lock.
/// `None` khi không có framework đã biết hoặc không có entry lock.)
pub fn resolve_web_framework(project_root: &Path) -> Result<Option<(String, String)>> {
    let manifest_path = project_root.join("package.json");
    if !manifest_path.exists() {
        return Ok(None);
    }
    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_path)
            .with_context(|| format!("cannot read {}", manifest_path.display()))?,
    )
    .with_context(|| format!("{} is not valid JSON", manifest_path.display()))?;
    let mut frameworks = Vec::new();
    if let Some(obj) = manifest.as_object() {
        for section in ["dependencies", "devDependencies"] {
            if let Some(deps) = obj.get(section).and_then(|v| v.as_object()) {
                for name in deps.keys() {
                    if let Some(id) = framework_id_for_package(name) {
                        frameworks.push(id.to_string());
                    }
                }
            }
        }
    }
    frameworks.sort();
    frameworks.dedup();
    if frameworks.len() != 1 {
        return Ok(None);
    }
    let Some(framework) = frameworks.into_iter().next() else {
        return Ok(None);
    };
    // Locked version: exact installed version binds the profile (Spec rev4 §6).
    // (Version đã lock: version cài chính xác ràng buộc profile (Spec rev4 §6).)
    let package_name = match framework.as_str() {
        "angular" => "@angular/core",
        "remix" => "@remix-run/react",
        _ => framework.as_str(),
    };
    let version = locked_packages(project_root)?
        .iter()
        .find(|(name, _)| name == package_name || name == &framework)
        .map(|(_, version)| version.clone());
    Ok(version.map(|v| (framework, v)))
}

/// Default profile store directory (user/machine scope).
/// Thư mục store profile mặc định (phạm vi user/máy).
pub fn default_profile_store_dir() -> std::path::PathBuf {
    mgc_platform::paths::mgc_data_dir().join("profiles")
}

/// Pre-flight profile gate (Spec rev4 §3.1 row 3): a lane whose active
/// profile observably requires package managers refuses upfront in native
/// mode — fail before spawn, never a mid-run kill. Lanes without a profile
/// pass through (level-2 tripwire-only, enforced at runtime).
/// (Cổng profile pre-flight (Spec rev4 §3.1 hàng 3): lane mà profile hiệu lực
/// cho thấy chắc chắn cần package manager thì native mode từ chối upfront —
/// fail trước spawn, không kill giữa chừng. Lane chưa profile thì qua
/// (mức 2 tripwire-only, thực thi lúc runtime).)
pub fn check_profile_gate(
    project_root: &Path,
    store_dir: &Path,
    operation: &str,
    compat_runner: Option<&str>,
) -> Result<()> {
    let Some((framework, version)) = resolve_web_framework(project_root)? else {
        return Ok(());
    };
    // Lock-bytes digest binds the profile to this exact lock (Spec rev4 §6).
    // (Digest byte lock ràng buộc profile vào đúng lock này (Spec rev4 §6).)
    let lock_bytes = std::fs::read(project_root.join("mgc.lock")).map_err(|e| {
        crate::error::stale_lock_needs_install(&format!("cannot read mgc.lock: {e}"))
    })?;
    let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(&lock_bytes).to_hex();
    let profile = match mgc_exec::spawn_profile::find_active_profile(
        store_dir,
        &framework,
        &version,
        std::env::consts::OS,
        operation,
        Some(&digest),
    ) {
        Ok(profile) => profile,
        Err(mgc_exec::spawn_profile::ProfileLookupError::Invalid(detail)) => {
            return Err(crate::error::invalid_spawn_profile(&detail));
        }
        Err(error) => return Err(crate::error::invalid_spawn_profile(&format!("{error:?}"))),
    };
    let Some(profile) = profile else {
        return Ok(());
    };
    if !profile.requires_pm.is_empty()
        && !profile
            .requires_pm
            .iter()
            .all(|required| compat_runner == Some(required.as_str()))
    {
        return Err(crate::error::lane_needs_compat_runner(
            &framework,
            &profile.requires_pm,
        ));
    }
    Ok(())
}

/// Extract `group:artifact` coordinates from a Maven `pom.xml`, matching
/// the resolver coordinate shape (`group:artifact` — see
/// `mgc-resolver/src/protocols/maven.rs`). Per-dependency `<groupId>` wins;
/// a missing one falls back to the project `<groupId>`.
/// (Trích tọa độ `group:artifact` từ `pom.xml` Maven, khớp dạng coordinate
/// của resolver. `<groupId>` từng dependency thắng; thiếu thì fallback
/// `<groupId>` project.)
///
/// Reserved: wired only where a native Maven lane writes `mgc.lock`
/// (follow-up per lane).
/// (Dành sẵn: chỉ nối nơi lane Maven native ghi `mgc.lock`
/// (follow-up từng lane).)
#[allow(dead_code)]
pub fn declared_maven_deps(pom_xml: &str) -> Vec<String> {
    fn text_of(block: &str, tag: &str) -> Option<String> {
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");
        let start = block.find(&open)? + open.len();
        let end = block[start..].find(&close)?;
        let value = block[start..start + end].trim();
        (!value.is_empty()).then(|| value.to_string())
    }
    // Project groupId fallback (first <groupId> outside <dependencies>).
    // (Fallback groupId project (<groupId> đầu ngoài <dependencies>).)
    let head = pom_xml.split("<dependencies>").next().unwrap_or("");
    let project_group = text_of(head, "groupId");
    let mut names = Vec::new();
    let Some(start) = pom_xml.find("<dependencies>") else {
        return names;
    };
    let mut rest = &pom_xml[start + "<dependencies>".len()..];
    let end = rest.find("</dependencies>").unwrap_or(rest.len());
    rest = &rest[..end];
    while let Some(dep_start) = rest.find("<dependency>") {
        rest = &rest[dep_start + "<dependency>".len()..];
        let dep_end = rest.find("</dependency>").unwrap_or(rest.len());
        let block = &rest[..dep_end];
        if let Some(artifact) = text_of(block, "artifactId") {
            let group = text_of(block, "groupId")
                .or_else(|| project_group.clone())
                .unwrap_or_default();
            if !group.is_empty() {
                names.push(format!("{group}:{artifact}"));
            }
        }
        rest = &rest[dep_end.min(rest.len())..];
    }
    names.sort();
    names.dedup();
    names
}

/// Extract `group:artifact` coordinates from Gradle `implementation`-family
/// declarations (`implementation("g:a:v")`, `implementation 'g:a'`).
/// (Trích tọa độ `group:artifact` từ khai báo họ `implementation` của
/// Gradle (`implementation("g:a:v")`, `implementation 'g:a'`).)
///
/// Reserved: same lock-shape caveat as [`declared_maven_deps`].
/// (Dành sẵn: cùng lưu ý dạng tên lock như [`declared_maven_deps`].)
#[allow(dead_code)]
pub fn declared_gradle_deps(build_gradle: &str) -> Vec<String> {
    const CONFIGS: &[&str] = &[
        "implementation",
        "api",
        "compileOnly",
        "runtimeOnly",
        "testImplementation",
        "testRuntimeOnly",
        "androidTestImplementation",
        "debugImplementation",
        "releaseImplementation",
    ];
    let mut names = Vec::new();
    for line in build_gradle.lines() {
        let line = line.trim().trim_end_matches(';').trim();
        let Some((config, rest)) = line.split_once([' ', '(', '\t']) else {
            continue;
        };
        if !CONFIGS.contains(&config.trim()) {
            continue;
        }
        let rest = rest.trim().trim_start_matches('(').trim();
        let quoted = rest
            .strip_prefix('"')
            .and_then(|s| s.split('"').next())
            .or_else(|| rest.strip_prefix('\'').and_then(|s| s.split('\'').next()));
        if let Some(coordinate) = quoted {
            let coordinate = coordinate.trim();
            if coordinate.contains(':') && !coordinate.is_empty() {
                names.push(coordinate.to_string());
            }
        }
    }
    names.sort();
    names.dedup();
    names
}

/// Extract `PackageReference` ids from MSBuild project files.
/// (Trích id `PackageReference` từ file project MSBuild.)
///
/// Reserved: same lock-shape caveat as [`declared_maven_deps`].
/// (Dành sẵn: cùng lưu ý dạng tên lock như [`declared_maven_deps`].)
#[allow(dead_code)]
pub fn declared_nuget_deps(csproj_xml: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = csproj_xml;
    while let Some(start) = rest.find("PackageReference") {
        rest = &rest[start + "PackageReference".len()..];
        let end = rest.find('>').unwrap_or(rest.len());
        let tag = &rest[..end];
        if let Some(include) = tag
            .split("Include=\"")
            .nth(1)
            .and_then(|s| s.split('"').next())
        {
            let include = include.trim();
            if !include.is_empty() {
                names.push(include.to_string());
            }
        }
        rest = &rest[end.min(rest.len())..];
    }
    names.sort();
    names.dedup();
    names
}

/// Extract Swift package identities matching adapter normalization
/// (lowercase; registry `id: "scope.name"` and git `url:` repo tails —
/// see `adapters/app/src/manifest/swift.rs`).
/// (Trích identity package Swift khớp chuẩn hóa adapter (lowercase;
/// `id: "scope.name"` registry và đuôi repo `url:` git).)
#[allow(dead_code)]
pub fn declared_swift_deps(package_swift: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = package_swift;
    while let Some(start) = rest.find(".package(") {
        rest = &rest[start + ".package(".len()..];
        let end = rest.find(')').unwrap_or(rest.len());
        let call = &rest[..end];
        // Registry form first: `.package(id: "scope.name", ...)`.
        // (Dạng registry trước: `.package(id: "scope.name", ...)`.)
        let identity = call
            .split("id:")
            .nth(1)
            .and_then(|s| {
                s.trim_start()
                    .strip_prefix('"')
                    .and_then(|s| s.split('"').next())
                    .map(str::to_string)
            })
            .or_else(|| {
                call.split("url:").nth(1).and_then(|s| {
                    let s = s.trim_start();
                    s.strip_prefix('"')
                        .and_then(|s| s.split('"').next())
                        .map(|url| {
                            url.trim_end_matches('/')
                                .trim_end_matches(".git")
                                .rsplit('/')
                                .next()
                                .unwrap_or_default()
                                .to_string()
                        })
                })
            });
        if let Some(identity) = identity {
            let identity = identity.to_lowercase();
            if !identity.is_empty() {
                names.push(identity);
            }
        }
        rest = &rest[end.min(rest.len())..];
    }
    names.sort();
    names.dedup();
    names
}

/// Declared Unity packages from `Packages/manifest.json` (`dependencies` keys).
/// (Package Unity khai báo từ `Packages/manifest.json` (key `dependencies`).)
///
/// Reserved: Unity lanes have no native `mgc.lock` writer yet (game adapter
/// only reads locks for SBOM), so wiring this would dead-end: the check
/// could never pass and `mgc install` could never fix it.
/// (Dành sẵn: lane Unity chưa ghi `mgc.lock` native (adapter game chỉ đọc
/// lock cho SBOM), nên nối sẽ bế tắc: check không bao giờ qua và
/// `mgc install` không bao giờ sửa được.)
#[allow(dead_code)]
pub fn declared_unity_deps(manifest_json: &str) -> Vec<String> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(manifest_json) else {
        return Vec::new();
    };
    let mut names: Vec<String> = value
        .get("dependencies")
        .and_then(|v| v.as_object())
        .map(|deps| deps.keys().cloned().collect())
        .unwrap_or_default();
    names.sort();
    names.dedup();
    names
}

/// Declared PlatformIO libraries from `platformio.ini` (`lib_deps` values,
/// first token each — owner/lib, URL or path as written).
/// (Thư viện PlatformIO khai báo từ `platformio.ini` (giá trị `lib_deps`,
/// token đầu mỗi cái).)
///
/// Reserved: same no-native-lock-writer caveat as [`declared_unity_deps`].
/// (Dành sẵn: cùng lưu ý chưa ghi lock native như [`declared_unity_deps`].)
#[allow(dead_code)]
pub fn declared_platformio_deps(platformio_ini: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_env = false;
    // Multi-line `lib_deps` values continue on following indented lines
    // until the next key or section.
    // (Giá trị `lib_deps` nhiều dòng nối ở dòng thụt đầu dòng sau, tới key
    // hoặc section tiếp theo.)
    let mut collecting = false;
    for raw_line in platformio_ini.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') {
            in_env = line.starts_with("[env:");
            collecting = false;
            continue;
        }
        if !in_env {
            continue;
        }
        let mut value = if let Some(rest) = line
            .strip_prefix("lib_deps")
            .and_then(|rest| rest.trim().strip_prefix('='))
        {
            collecting = true;
            rest
        } else if collecting && !line.is_empty() && !line.contains('=') {
            line
        } else {
            if !line.is_empty() {
                collecting = false;
            }
            continue;
        };
        value = value.trim();
        for token in value.split([',', ' ', '\t']).map(str::trim) {
            let token = token.trim_end_matches('\\').trim();
            // A lone `@` only separates `owner/lib @ version` pairs.
            // (`@` đơn chỉ ngăn cặp `owner/lib @ version`.)
            if token.is_empty() || token == "@" || token.starts_with([';', '#']) {
                continue;
            }
            names.push(token.to_string());
        }
    }
    names.sort();
    names.dedup();
    names
}

/// Declared Zephyr west projects from `west.yml` (`manifest.projects[].name`).
/// (Project west Zephyr khai báo từ `west.yml` (`manifest.projects[].name`).)
///
/// Reserved: same no-native-lock-writer caveat as [`declared_unity_deps`].
/// (Dành sẵn: cùng lưu ý chưa ghi lock native như [`declared_unity_deps`].)
#[allow(dead_code)]
pub fn declared_west_deps(west_yml: &str) -> Vec<String> {
    #[derive(serde::Deserialize)]
    struct WestManifest {
        #[serde(default)]
        manifest: WestBody,
    }
    #[derive(Default, serde::Deserialize)]
    struct WestBody {
        #[serde(default)]
        projects: Vec<WestProject>,
    }
    #[derive(serde::Deserialize)]
    struct WestProject {
        #[serde(default)]
        name: String,
    }
    let Ok(parsed) = serde_yaml::from_str::<WestManifest>(west_yml) else {
        return Vec::new();
    };
    let mut names: Vec<String> = parsed
        .manifest
        .projects
        .into_iter()
        .map(|p| p.name)
        .filter(|n| !n.is_empty())
        .collect();
    names.sort();
    names.dedup();
    names
}

/// Swift freshness for app/dev flows (`Package.swift`).
/// Freshness Swift cho luồng app/dev (`Package.swift`).
///
/// Only meaningful where the Swift lane writes `mgc.lock` natively;
/// registry-vs-git identity mismatches fail closed with an install hint.
/// (Chỉ có nghĩa nơi lane Swift ghi `mgc.lock` native; lệch identity
/// registry-vs-git fail-closed kèm hướng install.)
pub fn check_swift_freshness(project_root: &Path) -> Result<()> {
    check_manifest_file(
        project_root,
        "Package.swift",
        true,
        "app",
        mgc_lockfile::EcosystemTag::Swift,
        |content| Ok(declared_swift_deps(content)),
    )?;
    Ok(())
}

/// Declared npm dependencies from `package.json` (all four sections).
/// Dependency npm khai báo từ `package.json` (cả bốn mục).
pub fn declared_npm_deps(manifest: &serde_json::Value) -> Vec<String> {
    let mut declared = Vec::new();
    if let Some(obj) = manifest.as_object() {
        // Spec rev4 §8: the compared set is dependency metadata.
        // (Spec rev4 §8: tập so sánh là metadata dependency.)
        for section in [
            "dependencies",
            "devDependencies",
            "peerDependencies",
            "optionalDependencies",
        ] {
            if let Some(deps) = obj.get(section).and_then(|v| v.as_object()) {
                declared.extend(deps.keys().cloned());
            }
        }
    }
    declared.sort();
    declared.dedup();
    declared
}

/// Declared npm dependency version specs from all four package sections.
/// Version spec npm khai báo từ cả bốn mục dependency.
pub fn declared_npm_dep_specs(
    manifest: &serde_json::Value,
) -> Result<BTreeMap<String, Vec<String>>> {
    let Some(object) = manifest.as_object() else {
        return Err(crate::error::freshness_manifest_invalid(
            "package.json",
            "must be a JSON object",
        ));
    };
    let mut declared = BTreeMap::new();
    for section in [
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ] {
        let Some(value) = object.get(section) else {
            continue;
        };
        let Some(dependencies) = value.as_object() else {
            return Err(crate::error::freshness_manifest_invalid(
                "package.json",
                &format!("'{section}' must be an object"),
            ));
        };
        for (name, value) in dependencies {
            let Some(range) = value.as_str() else {
                return Err(crate::error::freshness_manifest_invalid(
                    "package.json",
                    &format!("dependency '{name}' must have a string version spec"),
                ));
            };
            declared
                .entry(name.clone())
                .or_insert_with(Vec::new)
                .push(range.to_string());
        }
    }
    Ok(declared)
}

/// Union pre-flight across every manifest the test runner understands
/// (Spec rev4 §8): each present manifest must match the lock. Manifests
/// without a verified lock-name shape (maven/nuget/gradle) stay out until
/// per-lane lock evidence exists.
/// (Pre-flight hợp mọi manifest mà test runner hiểu (Spec rev4 §8): mỗi
/// manifest hiện diện phải khớp lock. Manifest chưa xác minh dạng tên lock
/// (maven/nuget/gradle) đứng ngoài cho đến khi có evidence từng lane.)
pub fn check_any_freshness(project_root: &Path) -> Result<()> {
    // Absent lock means nothing to be consistent with (lockless projects,
    // e.g. optimizer fixtures, keep working); per-manifest `check_*`
    // functions used by dev entries still fail closed on a missing lock.
    // (Chưa có lock nghĩa là không có gì để nhất quán (project chưa lock,
    // vd fixture optimizer, vẫn chạy); hàm `check_*` từng manifest mà entry
    // dev dùng vẫn fail-closed khi thiếu lock.)
    if !project_root.join("mgc.lock").exists() {
        return Ok(());
    }
    // go.mod `replace` rewrites requirements the lock may not contain under
    // the original path — skip the go projection then (warn, don't guess).
    // (`replace` trong go.mod viết lại requirement mà lock có thể không chứa
    // dưới path gốc — khi đó bỏ qua projection go (cảnh báo, không đoán).)
    let go_path = project_root.join("go.mod");
    let go_has_replace = go_path.exists()
        && std::fs::read_to_string(&go_path)
            .map(|content| {
                content
                    .lines()
                    .any(|line| line.trim_start().starts_with("replace"))
            })
            .unwrap_or(false);
    if go_has_replace {
        mgc_ui::warning("go.mod has replace directives; skipping go freshness projection");
    }
    check_manifest_file(
        project_root,
        "package.json",
        false,
        "web",
        mgc_lockfile::EcosystemTag::Web,
        |content| {
            let value = serde_json::from_str(content).context("package.json is not valid JSON")?;
            Ok(declared_npm_deps(&value))
        },
    )?;
    check_manifest_file(
        project_root,
        "pyproject.toml",
        false,
        "ai",
        mgc_lockfile::EcosystemTag::Python,
        |content| Ok(declared_python_deps(content)),
    )?;
    check_flutter_manifest_freshness(project_root, false)?;
    check_manifest_file(
        project_root,
        "Cargo.toml",
        false,
        "game",
        mgc_lockfile::EcosystemTag::Rust,
        |content| Ok(declared_cargo_deps(content)),
    )?;
    if !go_has_replace {
        check_manifest_file(
            project_root,
            "go.mod",
            false,
            "lib",
            mgc_lockfile::EcosystemTag::Go,
            |content| Ok(declared_go_deps(content)),
        )?;
    }
    Ok(())
}
