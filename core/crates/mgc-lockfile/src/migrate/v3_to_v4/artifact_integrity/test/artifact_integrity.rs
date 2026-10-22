//! Verifies native SHA-256 hex conversion preserves the original bytes.
//! Xác minh đổi SHA-256 hex vẫn giữ nguyên bytes digest ban đầu.

use super::super::carry_install_sri;
use crate::Package;
use base64::Engine;

#[test]
fn converts_sha256_hex_to_installable_sri_without_rehashing() {
    let package = Package::new(
        "sample".into(),
        "1.0.0".into(),
        "https://example.invalid/sample.tgz".into(),
        format!("sha256-{}", "ab".repeat(32)),
    );
    let mut warnings = Vec::new();
    let artifact = carry_install_sri(&package, &mut warnings).expect("test operation must succeed");
    let sri = artifact.integrity_sri.expect("test operation must succeed");
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(
            sri.strip_prefix("sha256-")
                .expect("test operation must succeed"),
        )
        .expect("test operation must succeed");
    assert_eq!(bytes, vec![0xab; 32]);
    assert!(warnings.is_empty());
}
