//! Checks payload-preserving status cloning and equality.
//! Kiểm tra clone và equality status giữ nguyên payload.

use super::super::VerificationStatus;

#[test]
fn payload_status_clone_preserves_diagnostic() {
    let status = VerificationStatus::Tampered("digest mismatch".to_string());
    assert_eq!(status.clone(), status);
    assert!(
        matches!(status, VerificationStatus::Tampered(message) if message == "digest mismatch")
    );
}
