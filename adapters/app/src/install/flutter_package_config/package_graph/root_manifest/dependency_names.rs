//! Project sorted dependency names from a pubspec section.
//! Lấy tên dependency đã sắp xếp từ một section pubspec.

use super::super::FlutterPackageGraphError;

pub(super) fn flutter_pubspec_dependency_names(
    pubspec: &serde_yaml::Value,
    section: &str,
) -> Result<Vec<String>, FlutterPackageGraphError> {
    let Some(section_value) = pubspec.get(section) else {
        return Ok(Vec::new());
    };
    let mapping = section_value
        .as_mapping()
        .ok_or_else(|| FlutterPackageGraphError::RootDependencySectionNotMapping(section.into()))?;
    let mut names = mapping
        .keys()
        .map(|name| {
            name.as_str().map(str::to_string).ok_or_else(|| {
                FlutterPackageGraphError::NonStringRootDependencyName(section.into())
            })
        })
        .collect::<Result<Vec<_>, FlutterPackageGraphError>>()?;
    names.sort();
    Ok(names)
}
