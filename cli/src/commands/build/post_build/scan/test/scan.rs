//! The scan must catch injection sinks and stay quiet on clean output.
//! Scan phải bắt sink injection và yên với output sạch.

use super::super::{Severity, scan};

#[test]
fn a_clean_bundle_has_no_findings() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("app.js"),
        "console.log('hello');const x=eval('1+1');",
    )
    .unwrap();
    let report = scan(dir.path()).unwrap();
    assert!(report.findings.is_empty(), "{:?}", report.findings);
}

#[test]
fn eval_on_a_dynamic_argument_blocks() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.js"), "const y = eval(userInput);").unwrap();
    let report = scan(dir.path()).unwrap();
    assert!(report.has_blocking());
    assert!(report.findings.iter().any(|f| f.rule == "dynamic-eval"));
}

#[test]
fn a_development_build_is_noted_but_does_not_block() {
    // Shipping a dev build is a correctness problem, not an attack; it must be
    // visible without failing every build that has one.
    // Phát hành bản dev là vấn đề đúng đắn, không phải tấn công; nó phải hiện
    // ra mà không làm hỏng mọi build có nó.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("app.js"),
        "/* react-dom.development */ console.log(1);",
    )
    .unwrap();
    let report = scan(dir.path()).unwrap();
    let finding = report
        .findings
        .iter()
        .find(|f| f.rule == "development-build")
        .expect("development build must be noted");
    assert_eq!(finding.severity, Severity::Note);
    assert!(!report.has_blocking());
}

#[test]
fn binary_assets_are_not_read_as_text() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("logo.png"), [0x89, 0x50, 0x4e, 0x47, 0x00]).unwrap();
    let report = scan(dir.path()).unwrap();
    assert!(report.findings.is_empty());
}

/// A leaked build path is reported on every supported OS, not only macOS.
///
/// The rule used to test `/Users/` alone, so a Linux or Windows build machine
/// path shipped silently while the same leak on macOS was flagged.
/// (Đường dẫn build bị lộ phải được báo trên mọi OS, không chỉ macOS. Rule cũ chỉ
/// kiểm `/Users/` nên đường dẫn Linux/Windows lọt mà không báo.)
#[test]
fn absolute_build_paths_are_reported_on_every_platform() {
    for leaked in [
        "webpack:///Users/dev/project/src/index.ts",
        "webpack:///home/dev/project/src/index.ts",
        "webpack:///root/project/src/index.ts",
        "webpack://C:\\Users\\dev\\project\\src\\index.ts",
    ] {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("app.js"), leaked).unwrap();
        let report = scan(dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.rule == "absolute-source-path"),
            "a build path in '{leaked}' must be reported"
        );
    }
}

/// A home-looking path without the source-map marker stays quiet: the note is
/// about a source map leaking, not about any string that resembles a path.
/// (Đường dẫn trông như home nhưng không có marker source map thì không báo —
/// cảnh báo nói về source map lộ, không phải chuỗi giống path.)
#[test]
fn a_home_path_without_a_source_map_is_not_reported() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("app.js"),
        "const docs = 'see /home/dev/project/README.md';",
    )
    .unwrap();
    let report = scan(dir.path()).unwrap();
    assert!(
        !report
            .findings
            .iter()
            .any(|finding| finding.rule == "absolute-source-path"),
        "a plain string mentioning a home path is not a source-map leak"
    );
}

/// A standalone `.map` file is scanned: it is the artefact that carries the
/// leaked source paths, so excluding the extension made the rule unfireable in
/// the common (external map) case.
/// (File `.map` riêng lẻ được scan: nó chính là artefact mang đường dẫn bị lộ,
/// nên loại trừ extension làm rule không bắn được ở trường hợp phổ biến.)
#[test]
fn a_standalone_source_map_is_scanned() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("app.js.map"),
        r#"{"version":3,"sources":["webpack:///home/dev/app/src/main.ts"]}"#,
    )
    .unwrap();
    let report = scan(dir.path()).unwrap();
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.rule == "absolute-source-path"),
        "an external source map leaking a build path must be reported"
    );
}
