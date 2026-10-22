// Project lock discovery tests — test khám phá lockfile của dự án.
// Verify bounded-reader integration and deterministic Bun-before-Deno aggregation.
// Xác minh tích hợp bounded reader và thứ tự gộp Bun trước Deno ổn định.

use super::read_js_lockfiles;
use std::fs;

#[test]
fn merges_both_present_lockfiles_in_bun_then_deno_order() {
    let root = tempfile::tempdir().expect("create project lock fixture directory");
    fs::write(
        root.path().join("bun.lock"),
        r#"{"packages":{"alpha":["alpha@1.0.0","",{},""]}}"#,
    )
    .expect("write Bun lock fixture");
    fs::write(
        root.path().join("deno.lock"),
        r#"{"npm":{"beta@2.0.0":{}},"jsr":{"@std/bytes@0.224.0":{}}}"#,
    )
    .expect("write Deno lock fixture");

    let read = read_js_lockfiles(root.path()).expect("read project lock fixtures");
    let names: Vec<_> = read.npm_pins.iter().map(|pin| pin.name.as_str()).collect();
    assert_eq!(names, ["alpha", "beta"]);
    assert_eq!(read.jsr_skipped.len(), 1);
}
