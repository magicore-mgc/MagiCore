//! Wires general tar.gz extraction and nearest tests.
//! Nối giải nén tar.gz tổng quát và test gần nhất.

mod tar_extraction;
pub(super) use self::tar_extraction::extract_tar_gz;
#[cfg(test)]
mod test;
