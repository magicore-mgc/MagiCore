//! Sign a validated canonical BLAKE3 payload digest.
//! Ký digest payload BLAKE3 canonical đã kiểm tra.

use crate::{LockfileError, LockfileResult};

use crate::policy::digest::blake3_digest_bytes;

/// Sign a canonical payload digest with a keypair (returns the digest
/// string + the inline signature block; timestamps stay out-of-payload).
/// Ký digest payload canonical bằng keypair.
pub fn sign_payload_digest(
    digest: &str,
    key_pair: &mgc_crypto::keyring::KeyPair,
) -> LockfileResult<crate::v4::SignatureBlock> {
    let signer = key_pair.signer().map_err(LockfileError::CryptoError)?;
    let digest_bytes = blake3_digest_bytes(digest)?;
    let signature = signer.sign(&digest_bytes);
    let signature_str = format!("ed25519-{}", signature.to_base64());
    Ok(crate::v4::SignatureBlock {
        algorithm: "ed25519".to_string(),
        key_id: key_pair.key_id.clone(),
        public_key: signer.public_key().to_base64(),
        digest: digest.to_string(),
        signed_at: chrono::Utc::now().to_rfc3339(),
        signature: signature_str,
    })
}
