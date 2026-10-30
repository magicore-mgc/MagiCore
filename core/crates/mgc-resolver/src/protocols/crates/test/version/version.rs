use super::super::*;
use mgc_types::Version;

#[test]
fn cargo_caret_semantics() {
    let v = Version::parse("1.2.5").expect("test operation must succeed");
    assert!(cargo_matches("^1.2.3", &v));
    assert!(!cargo_matches(
        "^1.2.3",
        &Version::parse("2.0.0").expect("test operation must succeed")
    ));
    assert!(cargo_matches("1.2", &v)); // bare = caret
    assert!(cargo_matches(">=1.0, <2.0", &v));
    assert!(cargo_matches("~1.2.3", &v));
    assert!(!cargo_matches(
        "~1.2.3",
        &Version::parse("1.3.0").expect("test operation must succeed")
    ));
    assert!(cargo_matches("=1.2.5", &v));
    assert!(cargo_matches(
        "^0.25.0",
        &Version::parse("0.25.17+spec-1.1.0").expect("test operation must succeed")
    ));
}

/// Regression: specs that name a present version must match it regardless of
/// the bare/caret/exact/range spelling — the shapes a Cargo requirement can
/// take for an existing crate version.
/// Hồi quy: spec trỏ tới version đang có phải khớp bất kể dạng bare/caret/
/// exact/range — các dạng Cargo requirement có thể dùng cho version tồn tại.
#[test]
fn present_version_matches_all_spec_forms() {
    let v = Version::parse("3.20.0").expect("test operation must succeed");
    for range in [
        "3", "3.20", "3.20.0", "^3.20", "=3.20", "=3.20.0", ">=3.0,<4",
    ] {
        assert!(cargo_matches(range, &v), "spec '{range}' must match 3.20.0");
    }
    // Out-of-range specs still reject the same version (fail-closed intact).
    // Spec ngoài khoảng vẫn từ chối version đó (fail-closed giữ nguyên).
    for range in ["4", "^4.0", "=3.19.0", ">=4.0,<5"] {
        assert!(
            !cargo_matches(range, &v),
            "spec '{range}' must not match 3.20.0"
        );
    }
}
