use mgc_types::{MgError, MgResult};
use std::path::Path;

const MAX_AUDIT_PROJECT_INPUT_BYTES: u64 = 10 * 1024 * 1024;

/// Read project-controlled scanner input without following links or exceeding the byte ceiling.
/// Đọc input scanner do project cung cấp, chặn symlink và giới hạn kích thước.
pub(crate) fn read_bounded_project_input(path: &Path, label: &str) -> MgResult<String> {
    let bytes = mgc_lockfile::read_bounded_regular_file(path, MAX_AUDIT_PROJECT_INPUT_BYTES, label)
        .map_err(|error| {
            MgError::Other(format!(
                "failed to read {label} '{}': {error}",
                path.display()
            ))
        })?;
    String::from_utf8(bytes)
        .map_err(|error| MgError::Other(format!("{label} is not valid UTF-8: {error}")))
}
