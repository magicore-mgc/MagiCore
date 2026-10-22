//! Keyring storage wiring — Wiring storage keyring.
//! Each child owns one filesystem responsibility — Mỗi child sở hữu một trách nhiệm filesystem.

mod atomic_replace;
mod atomic_write;
mod file_guard;
mod path_guard;
mod read;
mod temporary_keyring_file;

pub(in crate::keyring) use atomic_write::write_private_file_atomic;
pub(in crate::keyring) use file_guard::{
    read_existing_for_backup, reject_non_regular_existing_path,
};
pub(in crate::keyring) use path_guard::validate_keyring_path;
pub(in crate::keyring) use read::read_keyring_contents;
