//! Wires Cargo archive root validation and nearest tests.
//! Nối xác minh root archive Cargo và các test gần nhất.

mod crate_extraction;
pub(super) use self::crate_extraction::extract_crate_tar_gz;
#[cfg(test)]
mod test;
