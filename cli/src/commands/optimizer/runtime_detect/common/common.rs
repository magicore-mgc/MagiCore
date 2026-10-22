//! Shared safe project-manifest reads.
use std::path::Path;

/// Read a regular project manifest without following a symlink.
/// Đọc manifest thường trong project, không đi theo symlink.
pub(in crate::commands::optimizer) fn project_text(
    project_root: &Path,
    name: &str,
) -> Option<String> {
    mgc_config::project::read_regular_project_text(&project_root.join(name), name)
        .ok()
        .flatten()
}
