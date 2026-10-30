use super::super::index::select_index_entry;
use super::super::*;

#[test]
fn sparse_prefix_rules() {
    assert_eq!(sparse_prefix("a"), "1");
    assert_eq!(sparse_prefix("ab"), "2");
    assert_eq!(sparse_prefix("abc"), "3/a");
    assert_eq!(sparse_prefix("serde"), "se/rd");
}

/// Build one index row with a bare 64-hex cksum.
/// Dựng một dòng index với cksum hex 64 ký tự trần.
fn entry(name: &str, vers: &str, yanked: bool) -> CrateIndexEntry {
    serde_json::from_str(&format!(
        r#"{{"name":"{name}","vers":"{vers}","deps":[],"cksum":"{}","features":{{}},"yanked":{yanked},"links":null}}"#,
        "a".repeat(64)
    ))
    .expect("valid sparse-index row")
}

/// Regression: every bare/caret/exact/range spec form must match a version
/// that exists in the index. The reported defect showed `idna = "3"`,
/// `"3.20"`, `"^3.20"`, `"=3.20"` and `">=3.0,<4"` failing even though a
/// matching version was present; this pins the selection to the highest
/// non-yanked match for each form.
/// Hồi quy: mọi dạng spec bare/caret/exact/range phải khớp version tồn tại.
/// Lỗi báo cáo cho thấy `idna = "3"`… fail dù có version khớp; test này ghim
/// việc chọn version non-yanked cao nhất cho từng dạng.
#[test]
fn existing_version_is_selected_for_every_spec_form() {
    let entries = vec![
        entry("idna", "0.5.0", false),
        entry("idna", "1.1.0", false),
        entry("idna", "3.18.0", false),
        entry("idna", "3.20.0", false),
    ];

    for range in ["3", "3.20", "^3.20", "=3.20", ">=3.0,<4"] {
        let selected = select_index_entry("idna", &[range.to_string()], entries.clone())
            .expect("a present version must match");
        assert_eq!(selected.vers, "3.20.0", "spec '{range}' must select 3.20.0");
    }
}

/// A wildcard still resolves to the highest present version.
/// Wildcard vẫn resolve về version cao nhất đang có.
#[test]
fn wildcard_resolves_to_highest_version() {
    let entries = vec![
        entry("idna", "1.1.0", false),
        entry("idna", "3.20.0", false),
        entry("idna", "4.0.0", false),
    ];
    let selected =
        select_index_entry("idna", &["*".to_string()], entries).expect("wildcard must resolve");
    assert_eq!(selected.vers, "4.0.0");
}

/// A genuinely-absent requirement stays fail-closed (no silent pick).
/// Yêu cầu thật sự vắng mặt vẫn fail-closed (không chọn ngầm).
#[test]
fn absent_version_fails_closed() {
    let entries = vec![entry("idna", "1.1.0", false), entry("idna", "0.5.0", false)];
    let error = select_index_entry("idna", &["3".to_string()], entries)
        .expect_err("requirement with no matching version must fail");
    assert!(
        error.to_string().contains("no non-yanked version"),
        "{error}"
    );
}

/// A yanked candidate is never selected, even when it is the only match.
/// Không bao giờ chọn ứng viên yanked, kể cả khi nó là match duy nhất.
#[test]
fn yanked_only_match_fails_closed() {
    let entries = vec![entry("idna", "3.20.0", true)];
    let error = select_index_entry("idna", &["3".to_string()], entries)
        .expect_err("only-yanked match must fail closed");
    assert!(
        error.to_string().contains("no non-yanked version"),
        "{error}"
    );
}
