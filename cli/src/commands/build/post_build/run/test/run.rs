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

#[test]
fn a_missing_output_directory_is_not_an_error() {
    let dir = tempfile::tempdir().unwrap();
    run(&dir.path().join("absent"), options("app", false)).expect("absent output is fine");
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
