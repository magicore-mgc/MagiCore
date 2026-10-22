//! Render and replace MagiCore-owned PATH marker blocks.
//! Tạo và thay block marker PATH do MagiCore sở hữu.

use std::path::Path;

#[cfg(not(windows))]
pub(in crate::paths::shell::unix) fn managed_path_profile_block(
    bin: &Path,
    fish: bool,
) -> anyhow::Result<String> {
    let path = bin
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("managed executable path is not valid UTF-8"))?;
    if path.chars().any(char::is_control) {
        anyhow::bail!("managed executable path contains control characters");
    }
    if fish {
        let quoted = path.replace('\\', "\\\\").replace('\'', "\\'");
        Ok(format!(
            "# >>> magicore-managed-tool-bin >>>\nif not contains -- '{quoted}' $PATH\n    set -gx PATH '{quoted}' $PATH\nend\n# <<< magicore-managed-tool-bin <<<"
        ))
    } else {
        let quoted = path.replace('\'', "'\\''");
        Ok(format!(
            "# >>> magicore-managed-tool-bin >>>\ncase \":$PATH:\" in\n  *:'{quoted}':*) ;;\n  *) export PATH='{quoted}':\"$PATH\" ;;\nesac\n# <<< magicore-managed-tool-bin <<<"
        ))
    }
}

#[cfg(not(windows))]
pub(in crate::paths::shell::unix) fn replace_managed_profile_block(
    current: &str,
    desired: &str,
) -> anyhow::Result<String> {
    const START: &str = "# >>> magicore-managed-tool-bin >>>";
    const END: &str = "# <<< magicore-managed-tool-bin <<<";
    let starts = current
        .match_indices(START)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let ends = current
        .match_indices(END)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    match (starts.as_slice(), ends.as_slice()) {
        ([], []) => {
            let mut updated = current.to_string();
            if !updated.is_empty() && !updated.ends_with('\n') {
                updated.push('\n');
            }
            if !updated.is_empty() {
                updated.push('\n');
            }
            updated.push_str(desired);
            updated.push('\n');
            Ok(updated)
        }
        ([start], [end]) if *end >= *start => {
            let end = current[*end..]
                .find('\n')
                .map(|offset| *end + offset + 1)
                .unwrap_or(current.len());
            let mut updated = String::with_capacity(current.len() + desired.len());
            updated.push_str(&current[..*start]);
            updated.push_str(desired);
            updated.push('\n');
            updated.push_str(&current[end..]);
            Ok(updated)
        }
        _ => anyhow::bail!("shell profile contains malformed or duplicate MagiCore PATH markers"),
    }
}
