//! Existing keyring file guard exports — Export guard file keyring hiện có.
//! Own backup reads and replacement checks — Sở hữu đọc backup và kiểm tra replace.

mod file_guard;
pub(in crate::keyring) use file_guard::{
    read_existing_for_backup, reject_non_regular_existing_path,
};

#[cfg(test)]
#[path = "test/file_guard.rs"]
mod tests;
