//! Formats the stable CLI outdated presentation.
//! Định dạng output CLI outdated ổn định.

use super::super::model::OutdatedPkg;

pub(in crate::commands::outdated) fn severity_label(
    current: &str,
    latest_major: u64,
) -> &'static str {
    let cur_major = current
        .trim_start_matches('^')
        .trim_start_matches('~')
        .split('.')
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    if latest_major > cur_major {
        "major"
    } else if latest_major == cur_major {
        "minor"
    } else {
        "patch"
    }
}

/// Format one outdated package row without writing to the terminal.
/// Định dạng một dòng package outdated mà không ghi ra terminal.
pub(in crate::commands::outdated) fn outdated_line(package: &OutdatedPkg) -> String {
    let severity = severity_label(&package.current, package.major);
    format!(
        "  {}: {} → {} ({})",
        package.name, package.current, package.latest, severity
    )
}

/// Format the stable summary after listing outdated packages.
/// Định dạng summary ổn định sau danh sách package outdated.
pub(in crate::commands::outdated) fn outdated_summary(count: usize) -> String {
    format!("{count} package(s) outdated. Run 'mgc update' to update.")
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
