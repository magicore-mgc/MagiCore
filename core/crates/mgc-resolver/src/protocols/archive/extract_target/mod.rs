//! Wires contained file/directory targets and nearest tests.
//! Nối xử lý target file/thư mục nằm trong root và test gần nhất.

mod extract_target;
pub(super) use self::extract_target::{ensure_extract_directory, ensure_extract_file_target};
#[cfg(test)]
mod test;
