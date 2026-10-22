//! Wires Cargo source hashing and its nearest test owner.
//! Nối owner hash source Cargo và owner test gần nhất.

mod cargo_source_hashes;
pub(super) use self::cargo_source_hashes::cargo_source_file_hashes;
#[cfg(test)]
mod test;
