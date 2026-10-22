//! Apply trust policy to already-computed verification facts.
//! Áp policy lên kết quả xác minh đã tính.

use crate::{LockfileError, LockfileResult};

use super::super::mode::LockPolicyMode;
use super::super::report::V4VerifyReport;

/// Enforce the signature policy on a verified report.
/// - tampered digest → ALWAYS fail (any mode).
/// - unsigned + require → fail; unsigned + warn → caller warns (Ok);
///   unsigned + off → Ok silently.
/// - bad signature → ALWAYS fail.
/// - foreign key + require → fail; foreign key + warn/off → Ok.
///
/// Áp chính sách chữ ký lên báo cáo đã verify.
pub fn enforce_policy(
    report: &V4VerifyReport,
    policy: LockPolicyMode,
    trust_keys: &[String],
) -> LockfileResult<()> {
    if !report.digest_ok {
        return Err(LockfileError::TamperedLockfile(
            "payload digest does not match metadata.lockfile_hash".to_string(),
        ));
    }
    if !report.signed {
        return match policy {
            LockPolicyMode::Require => Err(LockfileError::VerificationFailed(
                "unsigned lockfile rejected by policy `require`".to_string(),
            )),
            LockPolicyMode::Warn | LockPolicyMode::Off => Ok(()),
        };
    }
    if !report.signature_ok {
        return Err(LockfileError::InvalidSignatureFile(
            "ed25519 signature does not verify".to_string(),
        ));
    }
    let trusted = report
        .key_id
        .as_ref()
        .is_some_and(|id| trust_keys.iter().any(|k| k == id));
    if !trusted && policy == LockPolicyMode::Require {
        return Err(LockfileError::UntrustedKey(
            report.key_id.clone().unwrap_or_default(),
        ));
    }
    Ok(())
}
