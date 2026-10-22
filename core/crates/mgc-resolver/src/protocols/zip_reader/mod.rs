//! Minimal shared ZIP archive reader — bộ đọc archive ZIP tối giản dùng chung.
//! Giữ public facade cho native protocol và adapter.

mod central_entry;
mod crc32;
mod end_record;
mod extract_zip;
mod limits;
mod local_entry;
#[cfg(test)]
mod test_support;
mod zip_entry;
mod zip_path;
mod zip_reader;

pub use crc32::crc32;
pub use extract_zip::extract_zip;
pub use zip_entry::ZipEntry;
pub use zip_reader::read_zip_entries;
