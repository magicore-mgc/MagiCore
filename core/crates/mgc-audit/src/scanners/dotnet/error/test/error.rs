//! Verify malformed lock errors keep their display context and source.
//! Kiểm tra lỗi lock sai giữ context hiển thị và source gốc.

use super::*;

#[test]
fn lock_parse_error_keeps_serde_source_after_mapping() {
    let source = serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
    let error = MgError::from(DotnetLockParseError::from(source));

    let MgError::Anyhow(error) = error else {
        panic!("NuGet lock parse error lost its typed source chain");
    };
    assert!(error.to_string().starts_with("invalid packages.lock.json:"));
    assert!(error.chain().nth(1).is_some());
}
