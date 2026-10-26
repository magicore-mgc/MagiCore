//! Dev scanning finds the sources a build would refuse.
//! Scan dev tìm source mà build sẽ từ chối.

use super::super::scan_sources;

#[test]
fn a_clean_source_tree_reports_nothing() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("main.ts"), "console.log(eval('1+1'))").unwrap();
    assert!(scan_sources(dir.path()).unwrap().summary().is_none());
}

#[test]
fn a_dynamic_eval_in_source_is_named() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/bad.ts"), "const v = eval(input);").unwrap();
    let scan = scan_sources(dir.path()).unwrap();
    let summary = scan.summary().expect("a blocking source must be reported");
    assert!(summary.contains("src/bad.ts"), "{summary}");
}

#[test]
fn dependencies_and_build_output_are_ignored() {
    // A dependency's own eval is install's business; the dev banner is about
    // code the user is editing.
    // eval trong dependency là việc của install; banner dev nói về code user
    // đang sửa.
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("node_modules/pkg")).unwrap();
    std::fs::write(dir.path().join("node_modules/pkg/index.js"), "eval(x)").unwrap();
    std::fs::create_dir_all(dir.path().join("dist")).unwrap();
    std::fs::write(dir.path().join("dist/out.js"), "eval(x)").unwrap();
    assert!(scan_sources(dir.path()).unwrap().summary().is_none());
}
