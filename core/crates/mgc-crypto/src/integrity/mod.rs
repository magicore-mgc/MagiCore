//! Subresource integrity API — Giao diện toàn vẹn tài nguyên con.
//! Keep stable public value and verifier exports — Giữ export value và verifier public ổn định.

mod compute;
mod error;
mod file;
mod integrity;
mod sri_hash;
mod verify;

pub use integrity::IntegrityVerifier;
pub use sri_hash::SriHash;
