//! Verifies v4 digest math before adapters preserve sibling rows.
//! Xác minh toán digest v4 trước khi adapter giữ lại row sibling.

use crate::LockfileResult;

/// Verify a v4 merge base before an adapter preserves its sibling rows:
/// digest math must hold (Warn policy, no trust roots — same contract as
/// the web lane reader). A tampered base fails instead of being copied
/// verbatim into the new lock.
/// (Xác minh base v4 trước khi adapter giữ row khác của nó: toán digest
/// phải đúng (policy Warn, không trust root — cùng hợp đồng với reader
/// lane web). Base bị sửa thì fail thay vì copy nguyên vào lock mới.)
pub fn verify_v4_merge_base(doc: &crate::canonical::LockfileV4) -> LockfileResult<()> {
    let report = crate::policy::verify_v4_math(doc)?;
    crate::policy::enforce_policy(&report, crate::policy::LockPolicyMode::Warn, &[])?;
    Ok(())
}
