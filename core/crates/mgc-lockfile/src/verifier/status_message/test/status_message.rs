//! Checks stable user-facing verification text.
//! Kiểm tra message xác minh ổn định hiển thị cho người dùng.

use super::super::verification_status_message;
use crate::VerificationStatus;

#[test]
fn every_status_keeps_its_current_message() {
    assert_eq!(
        verification_status_message(&VerificationStatus::Valid),
        "✓ Lockfile signature valid"
    );
    assert_eq!(
        verification_status_message(&VerificationStatus::Unsigned),
        "WARN: Lockfile not signed — run 'mgc trust sign'"
    );
    assert_eq!(
        verification_status_message(&VerificationStatus::Tampered("hash".into())),
        "✗ Lockfile tampered: hash"
    );
    assert_eq!(
        verification_status_message(&VerificationStatus::InvalidSignature("bad".into())),
        "✗ Invalid signature: bad"
    );
    assert_eq!(
        verification_status_message(&VerificationStatus::UntrustedKey("key".into())),
        "WARN: Lockfile signer is not trusted: key"
    );
}
