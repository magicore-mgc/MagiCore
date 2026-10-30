//! Entry-point discovery precedence and server-entry coverage.
//! Độ ưu tiên tìm entry point và độ phủ entry server.
//!
//! These tests use a filesystem fixture that mirrors an Express generator
//! output (`app.js` + `"main":"app.js"`, or a bare `server.js`) so the reported
//! "Could not find entry point" defect is reproduced offline. No package
//! manager is spawned and no network is used.
//! Test dùng fixture filesystem mô phỏng output express-generator (`app.js` +
//! `"main":"app.js"`, hoặc `server.js` trần) để tái hiện lỗi "Could not find
//! entry point" ngoại tuyến. Không spawn package manager, không dùng mạng.

use super::*;

/// Write a fixture file under the project root, creating parents.
/// Ghi file fixture dưới project root, tạo thư mục cha nếu cần.
fn write(root: &Path, relative: &str, contents: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

#[test]
fn resolves_express_app_js_declared_as_main() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(
        root,
        "package.json",
        r#"{"name":"shop-api","main":"app.js","scripts":{"start":"node app.js"},"dependencies":{"express":"^4.0.0"}}"#,
    );
    write(root, "app.js", "const express = require('express');\n");

    let entry = find_entry_point(root).expect("express app.js project must resolve an entry");
    assert_eq!(entry, root.join("app.js"));
}

#[test]
fn resolves_bare_server_js_without_package_hint() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(
        root,
        "package.json",
        r#"{"name":"edge-api","scripts":{"start":"node server.js"},"dependencies":{"express":"^4.0.0"}}"#,
    );
    write(root, "server.js", "const http = require('node:http');\n");

    let entry = find_entry_point(root).expect("bare server.js project must resolve an entry");
    assert_eq!(entry, root.join("server.js"));
}

#[test]
fn resolves_src_server_js_when_root_has_none() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(
        root,
        "package.json",
        r#"{"name":"nested-api","scripts":{"start":"node src/server.js"}}"#,
    );
    write(root, "src/server.js", "// server entry\n");

    let entry = find_entry_point(root).expect("src/server.js project must resolve an entry");
    assert_eq!(entry, root.join("src/server.js"));
}

#[test]
fn resolves_lowercase_bin_object_first_path() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(
        root,
        "package.json",
        r#"{"name":"cli-server","bin":{"serve":"bin/serve.js"},"scripts":{"start":"node bin/serve.js"}}"#,
    );
    write(root, "bin/serve.js", "#!/usr/bin/env node\n");

    let entry = find_entry_point(root).expect("bin-only project must resolve an entry");
    assert_eq!(entry, root.join("bin/serve.js"));
}

#[test]
fn declared_candidates_precede_conventional_entries() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    // A declared `main` must win over a conventional `server.js` in the same
    // project, so a library-style declaration is still honored.
    // `main` được khai phải thắng `server.js` quy ước cùng project, để khai báo
    // kiểu thư viện vẫn được tôn trọng.
    write(
        root,
        "package.json",
        r#"{"name":"mixed","main":"bootstrap.js"}"#,
    );
    write(root, "bootstrap.js", "// declared entry\n");
    write(root, "server.js", "// conventional entry\n");

    let entry = find_entry_point(root).expect("declared main must resolve");
    assert_eq!(entry, root.join("bootstrap.js"));
}

#[test]
fn browser_entry_precedes_package_fields() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(root, "package.json", r#"{"name":"spa","main":"legacy.js"}"#);
    write(root, "main.js", "// browser entry\n");
    write(root, "legacy.js", "// stale declaration\n");

    let entry = find_entry_point(root).expect("browser candidate must resolve first");
    assert_eq!(entry, root.join("main.js"));
}

#[test]
fn reports_not_found_when_no_entry_exists() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(root, "package.json", r#"{"name":"empty","private":true}"#);

    let error = find_entry_point(root).expect_err("project with no entry must fail closed");
    assert!(error.to_string().contains("Could not find entry point"));
}

#[test]
fn declared_field_that_does_not_exist_falls_through() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    // `main` points at a not-yet-built output; the conventional `app.js` is the
    // real source and must still be selected.
    // `main` trỏ output chưa build; `app.js` quy ước mới là source thật và vẫn
    // phải được chọn.
    write(
        root,
        "package.json",
        r#"{"name":"dist-only","main":"dist/bundle.js"}"#,
    );
    write(root, "app.js", "// real source\n");

    let entry = find_entry_point(root).expect("fallback to app.js must resolve");
    assert_eq!(entry, root.join("app.js"));
}
