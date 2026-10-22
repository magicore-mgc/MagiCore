//! SHA-256 operations used by package marker identities.
//! Thao tác SHA-256 dùng cho định danh marker package.

mod digest;

pub use digest::{
    compute_sha256_hex, compute_sha256_hex_from_path, compute_sha256_hex_from_reader,
};

pub(crate) use digest::SHA256_READ_BUFFER_BYTES;
