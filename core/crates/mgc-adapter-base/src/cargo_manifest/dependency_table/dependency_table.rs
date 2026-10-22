//! Preserve Cargo-specific values while applying requested root dependencies.
//! Giữ giá trị riêng của Cargo khi cập nhật dependency root.

use super::super::error::{invalid_manifest, unsupported_mutation};
use mgc_types::Ecosystem;
use mgc_types::error::MgResult;
use mgc_types::package::DependencySpec;

pub(in crate::cargo_manifest) fn sync_dependency_table(
    root: &mut toml::Table,
    table_name: &str,
    desired: &[DependencySpec],
    ecosystem: Ecosystem,
) -> MgResult<()> {
    let desired_versions: std::collections::BTreeMap<String, String> = desired
        .iter()
        .map(|dependency| {
            let version = if dependency.range.is_star() {
                "*".to_string()
            } else {
                dependency.range.as_str().to_string()
            };
            (dependency.name.as_str().to_string(), version)
        })
        .collect();

    if !root.contains_key(table_name) {
        if desired_versions.is_empty() {
            return Ok(());
        }
        root.insert(
            table_name.to_string(),
            toml::Value::Table(toml::Table::new()),
        );
    }
    let table = root
        .get_mut(table_name)
        .and_then(toml::Value::as_table_mut)
        .ok_or_else(|| invalid_manifest(format!("Cargo.toml [{table_name}] is not a table")))?;

    let removed: Vec<String> = table
        .keys()
        .filter(|name| name.as_str() != "magicore" && !desired_versions.contains_key(*name))
        .cloned()
        .collect();
    for name in removed {
        table.remove(&name);
    }

    for (name, version) in desired_versions {
        match table.get_mut(&name) {
            Some(toml::Value::String(current)) => *current = version,
            Some(toml::Value::Table(specification)) => {
                if specification.contains_key("path")
                    || specification.contains_key("git")
                    || specification.contains_key("workspace")
                    || specification.contains_key("package")
                    || specification.contains_key("registry")
                {
                    return Err(unsupported_mutation(
                        ecosystem,
                        &format!(
                            "dependency '{name}' uses a Cargo path/git/workspace/alias/registry \
source that the current native resolver cannot preserve safely"
                        ),
                    ));
                }
                specification.insert("version".to_string(), toml::Value::String(version));
            }
            Some(_) => {
                return Err(invalid_manifest(format!(
                    "Cargo.toml dependency '{name}' has an unsupported specification"
                )));
            }
            None => {
                table.insert(name, toml::Value::String(version));
            }
        }
    }
    Ok(())
}
