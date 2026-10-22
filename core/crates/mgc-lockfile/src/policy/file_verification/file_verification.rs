//! Run the bounded parse → math → policy flow for an on-disk lockfile.
//! Chạy luồng parse có giới hạn → math → policy cho lock trên đĩa.

use std::path::Path;

use crate::canonical::LockfileV4;
use crate::{LockfileError, LockfileResult};

use super::super::enforcement::enforce_policy;
use super::super::mode::LockPolicyMode;
use super::super::report::V4VerifyReport;
use super::super::verification::verify_v4_math;

/// Verify a v4 lockfile on disk end to end (parse + math + policy).
/// Xác minh lockfile v4 trên đĩa trọn vẹn (parse + toán + policy).
pub fn verify_v4_file(
    path: &Path,
    policy: LockPolicyMode,
    trust_keys: &[String],
) -> LockfileResult<V4VerifyReport> {
    let bytes = crate::parser::read_lockfile_bytes(path)?;
    let content = std::str::from_utf8(&bytes).map_err(|error| {
        LockfileError::ParseError(format!("lockfile is not valid UTF-8: {error}"))
    })?;
    let lock: LockfileV4 = crate::canonical::parse_v4_document(content)?;
    let report = verify_v4_math(&lock)?;
    enforce_policy(&report, policy, trust_keys)?;
    Ok(report)
}
