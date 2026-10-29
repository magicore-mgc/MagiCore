//! Minimal shared ZIP archive reader — bộ đọc archive ZIP tối giản dùng chung.
//! Giữ public facade cho native protocol và adapter.

mod central_entry;
mod crc32;
mod end_record;
mod extract_zip;
mod limits;
mod local_entry;
#[cfg(test)]
#[cfg(test)]
#[path = "test/support/mod.rs"]
mod test_support;
mod zip_entry;
mod zip_path;
mod zip_reader;

#[cfg(test)]
#[path = "test/wcx3_install_hook_archives.rs"]
mod wcx3_install_hook_archives;

pub use crc32::crc32;
pub use extract_zip::{extract_zip, extract_zip_with_policy};
pub use zip_entry::ZipEntry;
pub use zip_reader::read_zip_entries;
