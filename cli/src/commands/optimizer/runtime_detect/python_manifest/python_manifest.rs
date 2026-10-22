//! Python dependency declaration and package-name normalization.
use super::super::project_text;
use std::path::Path;

/// Detect a normalized Python dependency in supported manifest sections.
/// Phát hiện dependency Python đã chuẩn hóa trong các vùng manifest được hỗ trợ.
pub(crate) fn python_project_declares_package(project_root: &Path, package: &str) -> bool {
    let normalized_package = normalize_python_package_name(package);
    requirements_file_declares_package(project_root.join("requirements.txt"), &normalized_package)
        || project_text(project_root, "pyproject.toml")
            .and_then(|content| toml::from_str::<toml::Value>(&content).ok())
            .is_some_and(|manifest| pyproject_declares_package(&manifest, &normalized_package))
}

fn requirements_file_declares_package(path: std::path::PathBuf, package: &str) -> bool {
    mgc_config::project::read_regular_project_text(&path, "requirements manifest")
        .ok()
        .flatten()
        .is_some_and(|content| {
            content.lines().any(|line| {
                let requirement = line.split('#').next().unwrap_or_default().trim();
                normalize_python_requirement(requirement) == package
            })
        })
}

fn pyproject_declares_package(manifest: &toml::Value, package: &str) -> bool {
    let project = manifest.get("project");
    let pep621_runtime = project
        .and_then(|value| value.get("dependencies"))
        .is_some_and(|value| dependency_array_contains(value, package));
    let pep621_optional = project
        .and_then(|value| value.get("optional-dependencies"))
        .is_some_and(|value| dependency_group_table_contains(value, package));
    let pep735_groups = manifest
        .get("dependency-groups")
        .is_some_and(|value| dependency_group_table_contains(value, package));
    let poetry = manifest.get("tool").and_then(|value| value.get("poetry"));
    let poetry_runtime = poetry
        .and_then(|value| value.get("dependencies"))
        .is_some_and(|value| dependency_key_table_contains(value, package));
    let poetry_groups = poetry
        .and_then(|value| value.get("group"))
        .is_some_and(|value| poetry_group_table_contains(value, package));

    pep621_runtime || pep621_optional || pep735_groups || poetry_runtime || poetry_groups
}

fn dependency_array_contains(value: &toml::Value, package: &str) -> bool {
    value.as_array().is_some_and(|dependencies| {
        dependencies.iter().any(|dependency| {
            dependency
                .as_str()
                .is_some_and(|requirement| normalize_python_requirement(requirement) == package)
        })
    })
}

fn dependency_group_table_contains(value: &toml::Value, package: &str) -> bool {
    value.as_table().is_some_and(|groups| {
        groups
            .values()
            .any(|dependencies| dependency_array_contains(dependencies, package))
    })
}

fn dependency_key_table_contains(value: &toml::Value, package: &str) -> bool {
    value.as_table().is_some_and(|dependencies| {
        dependencies
            .keys()
            .any(|dependency| normalize_python_package_name(dependency) == package)
    })
}

fn poetry_group_table_contains(value: &toml::Value, package: &str) -> bool {
    value.as_table().is_some_and(|groups| {
        groups.values().any(|group| {
            group
                .get("dependencies")
                .is_some_and(|dependencies| dependency_key_table_contains(dependencies, package))
        })
    })
}

fn normalize_python_requirement(requirement: &str) -> String {
    let name = requirement
        .split(|character: char| {
            character.is_whitespace()
                || matches!(character, '[' | '<' | '>' | '=' | '!' | '~' | '@' | ';')
        })
        .next()
        .unwrap_or_default();
    normalize_python_package_name(name)
}

fn normalize_python_package_name(package: &str) -> String {
    let mut normalized = String::with_capacity(package.len());
    let mut separator_pending = false;
    for character in package.trim().chars() {
        match character {
            '-' | '_' | '.' => separator_pending = !normalized.is_empty(),
            _ => {
                if separator_pending {
                    normalized.push('-');
                    separator_pending = false;
                }
                normalized.extend(character.to_lowercase());
            }
        }
    }
    normalized
}
