//! Validates external BLAKE3 text before it enters a CAS API.
//! Xác thực chuỗi BLAKE3 ngoài trước khi đi vào API CAS.

use crate::cas::integrity::validate_blake3_hex;
use crate::cas::store::StoreError;

/// Validate an external hash string before it reaches a CAS API.
/// Helper cho caller ngoài crate: validate hash trước khi chạm CAS API.
pub fn validate_external_hash(hash_hex: &str) -> Result<(), StoreError> {
    validate_blake3_hex(hash_hex)
        .map(|_| ())
        .map_err(|e| StoreError::InvalidHash(e.input))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
