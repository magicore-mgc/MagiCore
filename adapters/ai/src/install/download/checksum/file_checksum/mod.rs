//! File checksum owner wiring.
//! Wiring owner checksum file.

mod file_checksum;
pub use file_checksum::verify_file_checksum;

#[cfg(test)]
mod test;
