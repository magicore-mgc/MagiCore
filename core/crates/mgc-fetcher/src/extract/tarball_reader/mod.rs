//! Wires bounded reader extraction and its nearest tests.
//! Nối giải nén reader có giới hạn và test gần nhất.

mod tarball_reader;
pub use self::tarball_reader::extract_tarball_from_reader;
#[cfg(test)]
mod test;
