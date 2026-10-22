//! Recursive TOML version-reference counter.
//! Bộ đếm đệ quy version reference trong TOML.

/// Count matching `version.ref` values across tables and arrays.
/// Đếm các giá trị `version.ref` khớp bên trong table và array.
pub(in crate::manifest::gradle) fn count_version_ref_uses(
    value: &toml::Value,
    ref_name: &str,
) -> usize {
    match value {
        toml::Value::Table(table) => {
            let current = usize::from(
                table
                    .get("version")
                    .and_then(|version| version.get("ref"))
                    .and_then(toml::Value::as_str)
                    == Some(ref_name),
            );
            current
                + table
                    .values()
                    .map(|child| count_version_ref_uses(child, ref_name))
                    .sum::<usize>()
        }
        toml::Value::Array(items) => items
            .iter()
            .map(|child| count_version_ref_uses(child, ref_name))
            .sum(),
        _ => 0,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
