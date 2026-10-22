//! Dependency gate error contracts — contract lỗi dependency gate.
//! Compare exact text with existing diagnostics and prove the local type survives anyhow conversion.
//! So khớp text chẩn đoán cũ và xác nhận anyhow giữ kiểu lỗi cục bộ.

use crate::commands::dep_gate::DepGateError;

#[test]
fn local_dependency_gate_errors_keep_legacy_display_and_typed_identity() {
    let compat = DepGateError::compat_disabled("pip");
    assert_eq!(
        compat.to_string(),
        crate::error::dep_gate_compat_disabled("pip").to_string()
    );
    for (core, op, ecosystem) in [
        ("web", "install", Some("python")),
        ("hardware", "list", Some("rust")),
        ("cicd", "install", None),
    ] {
        assert_eq!(
            DepGateError::unsupported(core, op, ecosystem).to_string(),
            crate::error::dep_gate_unsupported(core, op, ecosystem).to_string()
        );
    }
    let mismatch =
        DepGateError::identity_mismatch("ai[python]", "install", "ai", "python", "lib", "rust");
    assert_eq!(
        mismatch.to_string(),
        "native adapter identity mismatch for ai[python] install: expected ai/python, got lib/rust"
    );
    let wrapped: anyhow::Error = mismatch.into();
    assert!(wrapped.downcast_ref::<DepGateError>().is_some());
}
