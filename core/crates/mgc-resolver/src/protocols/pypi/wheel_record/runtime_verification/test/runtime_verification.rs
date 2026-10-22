//! Ensure runtime validation trusts the authenticated archive RECORD only.
//! Bảo đảm runtime chỉ tin RECORD của archive đã xác thực.

use crate::protocols::pypi::PypiProtocol;
use crate::protocols::pypi::wheel_record::test_support::{entry_for, stored_zip};
use crate::protocols::sha256_hex;

#[test]
fn runtime_verification_uses_authenticated_wheel_record_not_mutable_site_record() {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let six_py = b"VALUE = 'trusted'\n";
    let meta = b"Name: six\nVersion: 1.17.0\n";
    let record = format!(
        "six.py,sha256={},{}\nsix-1.17.0.dist-info/METADATA,sha256={},{}\nsix-1.17.0.dist-info/RECORD,,\n",
        b64.encode(Sha256::digest(six_py)),
        six_py.len(),
        b64.encode(Sha256::digest(meta)),
        meta.len(),
    );
    let wheel = stored_zip(&[
        ("six.py", six_py.as_slice()),
        ("six-1.17.0.dist-info/METADATA", meta.as_slice()),
        ("six-1.17.0.dist-info/RECORD", record.as_bytes()),
    ]);
    let digest = sha256_hex(&wheel);
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let site = PypiProtocol::new("https://pypi.org")
        .materialize_importable(
            &entry_for("https://files.pythonhosted.org/six-1.17.0-py3-none-any.whl"),
            &wheel,
            dir.path(),
        )
        .expect("test operation must succeed")
        .expect("test operation must succeed");

    PypiProtocol::verify_runtime_materialization(&wheel, "six", "1.17.0", &digest, &site)
        .expect("test operation must succeed");
    // The old normalization erased separators and treated `six` and
    // `s-ix` as identical; PEP 503 keeps one canonical hyphen.
    // (Chuẩn hóa cũ xóa dấu phân cách khiến `six` trùng `s-ix`.)
    assert!(
        PypiProtocol::verify_runtime_materialization(&wheel, "s-ix", "1.17.0", &digest, &site)
            .is_err()
    );
    assert!(
        PypiProtocol::verify_runtime_materialization(
            &wheel,
            "attacker-package",
            "1.17.0",
            &digest,
            &site
        )
        .is_err()
    );

    std::fs::write(site.join("six.py"), b"VALUE = 'attacker-controlled'\n")
        .expect("test operation must succeed");
    // Rewriting the extracted RECORD must not bless the changed module.
    // Sửa RECORD đã giải nén không được hợp thức hóa module đã bị đổi.
    let changed = b"VALUE = 'attacker-controlled'\n";
    let forged_record = format!(
        "six.py,sha256={},{}\nsix-1.17.0.dist-info/METADATA,sha256={},{}\nsix-1.17.0.dist-info/RECORD,,\n",
        b64.encode(Sha256::digest(changed)),
        changed.len(),
        b64.encode(Sha256::digest(meta)),
        meta.len(),
    );
    std::fs::write(site.join("six-1.17.0.dist-info/RECORD"), forged_record)
        .expect("test operation must succeed");
    assert!(
        PypiProtocol::verify_runtime_materialization(&wheel, "six", "1.17.0", &digest, &site)
            .is_err()
    );
}
