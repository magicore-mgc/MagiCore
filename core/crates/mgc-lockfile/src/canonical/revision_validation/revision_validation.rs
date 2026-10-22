//! Rejects v4 additive fields when the declared revision does not sign them.
//! Từ chối field mở rộng khi revision khai báo chưa đưa chúng vào chữ ký.

use crate::canonical::LockfileV4;
/// Validate that additive fields are only used by a revision that signs them.
/// Xác minh field mới chỉ xuất hiện ở revision có ký chúng vào payload.
pub(crate) fn validate_v4_revision(lock: &LockfileV4) -> crate::LockfileResult<()> {
    if !crate::v4::is_v4_schema_version(&lock.version) {
        return Err(crate::LockfileError::ParseError(format!(
            "expected a supported v4 lockfile revision, got version '{}'",
            lock.version
        )));
    }
    if lock.version == crate::v4::LOCKFILE_SCHEMA_V4_LEGACY
        && lock
            .packages
            .iter()
            .any(|package| package.markers.is_some())
    {
        return Err(crate::LockfileError::ParseError(
            "package markers require lockfile schema revision 4.1".to_string(),
        ));
    }
    Ok(())
}
