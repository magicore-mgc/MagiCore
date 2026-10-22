//! Shared registry digest exports.
//! Export digest registry dùng chung.

mod digest;
pub use digest::{sha256_hex, store_ref_for_blake3};
