//! Maps verification status values to stable console messages.
//! Ánh xạ status xác minh thành message console ổn định.

use super::super::status::VerificationStatus;

/// Get the user-facing verification status message.
/// Lấy message status xác minh hiển thị cho người dùng.
pub fn verification_status_message(status: &VerificationStatus) -> String {
    match status {
        VerificationStatus::Valid => "✓ Lockfile signature valid".to_string(),
        VerificationStatus::Unsigned => {
            "WARN: Lockfile not signed — run 'mgc trust sign'".to_string()
        }
        VerificationStatus::Tampered(message) => format!("✗ Lockfile tampered: {message}"),
        VerificationStatus::InvalidSignature(message) => {
            format!("✗ Invalid signature: {message}")
        }
        VerificationStatus::UntrustedKey(key_id) => {
            format!("WARN: Lockfile signer is not trusted: {key_id}")
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
