//! Verify Pub archive bytes against their SHA-256 cache key.
//! Xác minh byte archive Pub theo khóa cache SHA-256.
use mgc_types::{MgError, MgResult};
use sha2::{Digest, Sha256};

pub(in crate::install::pub_archive_cache) fn verify(expected: &str, bytes: &[u8]) -> MgResult<()> {
    if !hex::encode(Sha256::digest(bytes)).eq_ignore_ascii_case(expected) {
        return Err(MgError::Integrity(
            "cached Pub archive SHA-256 mismatch".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
