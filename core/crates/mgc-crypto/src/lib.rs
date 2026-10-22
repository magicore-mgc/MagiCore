//! `mgc-crypto` — Cryptographic primitives for MagiCore supply chain security
//! Cryptographic primitives cho bảo mật chuỗi cung ứng MagiCore
//!
//! Provides BLAKE3 hashing, Ed25519 signing/verification, keyring management and SIMD-accelerated operations.
//! Cung cấp hashing BLAKE3, ký/xác minh Ed25519, quản lý keyring và thao tác tăng tốc SIMD.

pub mod blake3_signer;
pub mod checksum;
pub mod ed25519_signer;
mod error;
pub mod integrity;
pub mod keyring;
mod result;
pub mod simd;

pub use blake3_signer::{Blake3Hash, Blake3Hasher};
pub use ed25519_signer::{Ed25519PublicKey, Ed25519Signature, Ed25519Signer};
pub use error::CryptoError;
pub use integrity::{IntegrityVerifier, SriHash};
pub use keyring::{KeyPair, Keyring};
pub use result::CryptoResult;
pub use simd::{SimdCapability, detect_simd};
