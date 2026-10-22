//! Wires staged atomic file publication and nearest tests.
//! Nối ghi file staging nguyên tử và các test gần nhất.

mod extracted_file_write;
pub(super) use self::extracted_file_write::write_extracted_file;
#[cfg(test)]
mod test;
