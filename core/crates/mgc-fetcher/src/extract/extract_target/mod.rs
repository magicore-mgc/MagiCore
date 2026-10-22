//! Wires destination root and target safety checks.
//! Nối kiểm tra an toàn cho root và target đích.

mod extract_target;
pub(super) use self::extract_target::{
    ensure_extract_directory, ensure_extract_file_target, ensure_extract_root,
};
#[cfg(test)]
mod test;
