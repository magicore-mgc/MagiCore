//! Read and validate Cargo.toml before constructing a shared Manifest.
//! Đọc và kiểm tra Cargo.toml trước khi dựng Manifest dùng chung.

use super::super::error::{parse_error, unsupported_manifest};
use mgc_types::error::MgResult;
use mgc_types::package::{DependencySpec, PackageName, VersionRange};
use mgc_types::{Ecosystem, Manifest, Version};
use std::path::Path;

pub(in crate::cargo_manifest) fn parse_manifest(
    root: &Path,
    ecosystem: Ecosystem,
) -> MgResult<Manifest> {
    let content = crate::project_file::read_regular_text(&root.join("Cargo.toml"), "Cargo.toml")?;
    let value: toml::Value = toml::from_str(&content).map_err(parse_error)?;
    let name = value
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let mut manifest = Manifest::new(&name, ecosystem);
    manifest.version = value
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(toml::Value::as_str)
        .and_then(|version| Version::parse(version).ok());

    // The current lock/resolver model cannot represent build or
    // target-specific dependency groups without producing an incomplete graph.
    // (Resolver hiện chưa biểu diễn build/target dependencies an toàn.)
    if value
        .get("build-dependencies")
        .and_then(toml::Value::as_table)
        .is_some_and(|table| !table.is_empty())
    {
        return Err(unsupported_manifest(
            ecosystem,
            "Cargo build-dependencies are not yet supported by the native resolver",
        ));
    }
    if value
        .get("target")
        .and_then(toml::Value::as_table)
        .is_some_and(|targets| {
            targets.values().any(|target| {
                ["dependencies", "dev-dependencies", "build-dependencies"]
                    .into_iter()
                    .any(|key| {
                        target
                            .get(key)
                            .and_then(toml::Value::as_table)
                            .is_some_and(|table| !table.is_empty())
                    })
            })
        })
    {
        return Err(unsupported_manifest(
            ecosystem,
            "Cargo target-specific dependencies are not yet supported by the native resolver",
        ));
    }
    if value
        .get("features")
        .and_then(toml::Value::as_table)
        .is_some_and(|features| {
            features.values().any(|value| match value {
                toml::Value::Array(items) => !items.is_empty(),
                toml::Value::Table(items) => !items.is_empty(),
                _ => true,
            })
        })
    {
        return Err(unsupported_manifest(
            ecosystem,
            "Cargo feature activation is not yet modeled by the native resolver",
        ));
    }

    let mut add_dependencies = |table: Option<&toml::Value>, dev: bool| -> MgResult<()> {
        let Some(table) = table else { return Ok(()) };
        let Some(dependencies) = table.as_table() else {
            return Ok(());
        };
        for (dep_name, specification) in dependencies {
            if dep_name == "magicore" {
                continue;
            }
            let (range, features, default_features) = match specification {
                toml::Value::String(range) => (range.clone(), Vec::new(), true),
                toml::Value::Table(specification) => {
                    if ["path", "git", "workspace", "package", "registry"]
                        .iter()
                        .any(|key| specification.contains_key(*key))
                    {
                        return Err(unsupported_manifest(
                            ecosystem,
                            &format!(
                                "Cargo dependency '{dep_name}' uses a path/git/workspace/alias/registry \
source that the native resolver does not support"
                            ),
                        ));
                    }
                    if specification.get("optional").and_then(toml::Value::as_bool) == Some(true) {
                        return Err(unsupported_manifest(
                            ecosystem,
                            &format!(
                                "optional Cargo dependency '{dep_name}' is not activated by the root feature resolver yet"
                            ),
                        ));
                    }
                    let range = specification
                        .get("version")
                        .and_then(toml::Value::as_str)
                        .ok_or_else(|| {
                            unsupported_manifest(ecosystem, &format!(
                                "Cargo dependency '{dep_name}' has no registry version requirement"
                            ))
                        })?
                        .to_string();
                    let features = specification
                        .get("features")
                        .map(|value| {
                            value
                                .as_array()
                                .ok_or_else(|| {
                                    unsupported_manifest(ecosystem, &format!(
                                        "Cargo dependency '{dep_name}' features must be an array of strings"
                                    ))
                                })?
                                .iter()
                                .map(|feature| {
                                    feature
                                        .as_str()
                                        .filter(|feature| !feature.trim().is_empty())
                                        .map(str::to_string)
                                        .ok_or_else(|| {
                                            unsupported_manifest(ecosystem, &format!(
                                                "Cargo dependency '{dep_name}' features must be non-empty strings"
                                            ))
                                        })
                                })
                                .collect::<MgResult<Vec<_>>>()
                        })
                        .transpose()?
                        .unwrap_or_default();
                    let default_features = specification
                        .get("default-features")
                        .map(|value| {
                            value.as_bool().ok_or_else(|| {
                                unsupported_manifest(ecosystem, &format!(
                                    "Cargo dependency '{dep_name}' default-features must be a boolean"
                                ))
                            })
                        })
                        .transpose()?
                        .unwrap_or(true);
                    (range, features, default_features)
                }
                _ => {
                    return Err(unsupported_manifest(
                        ecosystem,
                        &format!(
                            "Cargo dependency '{dep_name}' must be a version string or version table"
                        ),
                    ));
                }
            };
            let mut dependency =
                DependencySpec::new(PackageName::new(dep_name)?, VersionRange::parse(&range)?);
            dependency.dev = dev;
            dependency.features = features;
            dependency.default_features = default_features;
            manifest.add_dep(dependency, dev, false, false);
        }
        Ok(())
    };
    add_dependencies(value.get("dependencies"), false)?;
    add_dependencies(value.get("dev-dependencies"), true)?;
    Ok(manifest)
}
