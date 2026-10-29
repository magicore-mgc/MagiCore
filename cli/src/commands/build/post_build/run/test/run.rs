//! The runner describes, scans and signs any output directory.
//! Runner mô tả, scan và ký mọi thư mục output.

use super::super::{PostBuildOptions, run};

fn options<'a>(name: &'a str, embed: bool) -> PostBuildOptions<'a> {
    PostBuildOptions {
        name,
        version: "1.0.0",
        fail_on_findings: true,
        embed_identity: embed,
    }
}

/// W-1 b: a build that reports a missing output directory as success ships an
/// SBOM, scan and signature that describe nothing.
/// (W-1 b: build báo thành công khi thư mục output không tồn tại sẽ phát hành
/// SBOM/scan/chữ ký mô tả hư không.)
#[test]
fn a_missing_output_directory_fails_the_build() {
    let dir = tempfile::tempdir().unwrap();
    let absent = dir.path().join("absent");
    let error = run(&absent, options("app", false)).expect_err("absent output must fail");
    let message = error.to_string();
    assert!(message.contains("post-build failed"), "{message}");
    assert!(
        message.contains("absent"),
        "the message must name the path: {message}"
    );
}

#[test]
fn a_plain_binary_output_is_described_and_hashed() {
    // A compiled artefact has no script to prepend to, so identity embedding is
    // off and the SBOM alone describes it.
    // Artefact đã biên dịch không có script để chèn, nên tắt nhúng danh tính và
    // chỉ SBOM mô tả nó.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app"), b"\x7fELF").unwrap();
    run(dir.path(), options("app", false)).expect("a binary output is handled");

    let sbom: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.path().join("sbom.output.cdx.json")).unwrap())
            .unwrap();
    assert_eq!(sbom["components"].as_array().unwrap().len(), 1);
    assert_eq!(sbom["components"][0]["name"], "app");
}

#[test]
fn a_blocking_finding_is_refused_unless_the_caller_opts_out() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("bundle.js"), "eval(userInput)").unwrap();

    let error = run(dir.path(), options("app", false)).expect_err("a blocking finding must fail");
    assert!(error.to_string().contains("security scan"), "{error}");

    let mut tolerant = options("app", false);
    tolerant.fail_on_findings = false;
    run(dir.path(), tolerant).expect("the caller may opt out");
}
