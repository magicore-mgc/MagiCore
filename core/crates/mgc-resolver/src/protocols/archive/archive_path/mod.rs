//! Wires archive path validation helpers and their nearest tests.
//! Nối các helper xác minh path archive cùng test gần nhất.

mod archive_path;
pub(super) use self::archive_path::{
    ensure_directory_component, sanitize_archive_path, split_safe_relative,
};
#[cfg(test)]
mod test;
