//! Wires shared archive expansion limits.
//! Nối các giới hạn bung archive dùng chung.

mod archive_limits;
pub(in crate::extract) use self::archive_limits::{
    FORBIDDEN_MODE_BITS, NUGET_INSTALL_SCRIPT_BASENAMES, PYTHON_SITE_HOOK_MODULES,
    mode_declares_symlink,
};
pub(super) use self::archive_limits::{MAX_ARCHIVE_ENTRIES, MAX_ENTRY_BYTES, MAX_TOTAL_FILE_BYTES};
#[cfg(test)]
mod test;
