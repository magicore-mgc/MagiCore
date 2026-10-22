//! Adler-32 checksum API — API checksum Adler-32.
//! Keep the stable public functions at this module boundary — Giữ hàm public ổn định tại boundary này.

mod checksum;
mod verification;

pub use checksum::checksum_adler32;
pub use verification::verify_adler32;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
