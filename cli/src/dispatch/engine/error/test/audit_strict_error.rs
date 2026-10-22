// Audit-strict error conversion — retain delegation to the shared audit helper.
// Chuyển lỗi audit-strict — giữ nguyên ủy quyền sang audit helper dùng chung.
use super::super::EnginePolicyError;

#[test]
fn unsupported_audit_strict_command_uses_the_shared_diagnostic() {
    assert_eq!(
        EnginePolicyError::AuditStrictUnsupported { command: "build" }
            .into_error()
            .to_string(),
        "--audit-strict has no effect on 'build' (strict audit applies to: audit, verify, install, add and their per-core variants)"
    );
}
