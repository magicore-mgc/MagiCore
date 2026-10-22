//! Unix executable bits and conservative non-Unix behavior.
//! Bit executable Unix và hành vi bảo thủ trên hệ không phải Unix.

#[cfg(unix)]
pub(crate) fn mode_is_executable(mode: u32) -> bool {
    mode & 0o111 != 0
}

#[cfg(not(unix))]
pub(crate) fn mode_is_executable(_mode: u32) -> bool {
    false
}

#[cfg(unix)]
pub(crate) fn metadata_is_executable(metadata: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
pub(crate) fn metadata_is_executable(_metadata: &std::fs::Metadata) -> bool {
    false
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
