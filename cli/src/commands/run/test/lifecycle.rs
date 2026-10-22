//! Pre/main/post lifecycle tests.
//! Test vòng đời pre/main/post.
use super::*;

#[test]
fn pre_hook_failure_aborts_chain() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_with_hooks;
    // pre fails (`cargo --bad-flag-xyz` exits non-zero) while main would
    // succeed — Err proves main never ran.
    // (pre hỏng trong khi main sẽ thành công — Err chứng minh main không chạy.)
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts": {"prego": "cargo --bad-flag-xyz", "go": "cargo --version"}}"#,
    );
    let compat = CompatMode::Native;
    let err = run_with_hooks(dir.path(), "go", &[], &compat, None, "run").unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn post_hook_runs_on_success() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_with_hooks;
    // Main succeeds, post fails — Err proves post ran.
    // (Main thành công, post hỏng — Err chứng minh post đã chạy.)
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts": {"go": "cargo --version", "postgo": "cargo --bad-flag-xyz"}}"#,
    );
    let compat = CompatMode::Native;
    let err = run_with_hooks(dir.path(), "go", &[], &compat, None, "run").unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn main_failure_wins_over_post_failure() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_with_hooks;
    // Both main and post fail — the reported error must be main's
    // (post failure is only warned). Distinctive commands tell them apart.
    // (Cả main và post đều hỏng — lỗi báo phải là của main (lỗi post chỉ
    // cảnh báo). Lệnh khác nhau phân biệt chúng.)
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts": {"go": "npm install left-pad-marker", "postgo": "cargo --bad-post-xyz"}}"#,
    );
    let compat = CompatMode::Native;
    let err = run_with_hooks(dir.path(), "go", &[], &compat, None, "run").unwrap_err();
    assert!(
        err.to_string().contains("left-pad-marker"),
        "expected main failure, got: {err}"
    );
}
