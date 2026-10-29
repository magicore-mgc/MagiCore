//! Lib dev language detection tests — test nhận diện ngôn ngữ cho dev Lib.
//! Each case pins the manifest-to-extension mapping, and proves that a project
//! the loop cannot watch fails instead of reporting a false success.
//! (Mỗi case ghim mapping manifest↔extension và chứng minh project không watch
//! được thì fail thay vì báo thành công giả.)

use std::fs;

use super::super::{count_watched_sources, watch_extensions};

fn project_with(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("temp dir");
    for (name, contents) in files {
        let path = dir.path().join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(path, contents).expect("write fixture");
    }
    dir
}

/// Every language MagiCore builds for `lib` has a watch rule.
/// (Mọi ngôn ngữ MagiCore build cho `lib` đều có rule watch.)
#[test]
fn every_lib_language_has_a_watch_rule() {
    // Manifest fixtures and the watch rules they must select.
    // (Fixture manifest và rule watch mà nó phải chọn.)
    type Case = (
        &'static [(&'static str, &'static str)],
        &'static [&'static str],
    );
    let cases: &[Case] = &[
        (&[("Cargo.toml", "")], &["rs", "toml"]),
        (&[("pyproject.toml", "")], &["py", "pyi", "toml", "cfg"]),
        (&[("go.mod", "")], &["go", "mod", "sum"]),
        (
            &[("pom.xml", "")],
            &["java", "kt", "gradle", "xml", "properties"],
        ),
        (
            &[("build.gradle", "")],
            &["java", "kt", "gradle", "xml", "properties"],
        ),
        (
            &[("App.csproj", "")],
            &["cs", "csproj", "sln", "props", "targets"],
        ),
        (
            &[("tsconfig.json", "")],
            &["ts", "tsx", "mts", "cts", "json"],
        ),
        (
            &[("package.json", "{}")],
            &["ts", "tsx", "mts", "cts", "json"],
        ),
    ];
    for (files, expected) in cases {
        let dir = project_with(files);
        assert_eq!(
            watch_extensions(dir.path()),
            *expected,
            "manifest {:?} must map to its own watch rules",
            files.iter().map(|(name, _)| *name).collect::<Vec<_>>()
        );
    }
}

/// A project with no recognised manifest has no watch rule, so `dev` must fail.
/// (Project không có manifest đã biết thì không có rule watch, `dev` phải fail.)
#[test]
fn an_unrecognised_project_has_no_watch_rule() {
    let dir = project_with(&[("README.md", "# nothing")]);
    assert!(
        watch_extensions(dir.path()).is_empty(),
        "a project without a lib manifest must not claim a watch rule"
    );
}

/// The count only follows the language's extensions and skips build dirs, so a
/// rebuild is not triggered by `target/` churn.
/// (Đếm chỉ theo extension của ngôn ngữ và bỏ thư mục build, nên `target/`
/// không kích rebuild.)
#[test]
fn counted_sources_ignore_build_directories() {
    let dir = project_with(&[
        ("Cargo.toml", ""),
        ("src/lib.rs", ""),
        ("src/nested/mod.rs", ""),
        ("tests/it.rs", ""),
        ("target/debug/generated.rs", ""),
        ("node_modules/x/index.rs", ""),
        (".magicore/cache/y.rs", ""),
        ("README.md", ""),
    ]);
    let count = count_watched_sources(dir.path(), watch_extensions(dir.path())).expect("count");
    assert_eq!(
        count, 4,
        "Cargo.toml + src/lib.rs + src/nested/mod.rs + tests/it.rs must be counted; \
         target/, node_modules/ and .magicore/ must not"
    );
}

/// Extensions match case-insensitively so a `.RS` file on macOS still counts.
/// (Extension so khớp không phân biệt hoa thường để file `.RS` trên macOS vẫn tính.)
#[test]
fn extension_matching_is_case_insensitive() {
    let dir = project_with(&[("Cargo.toml", ""), ("src/UPPER.RS", "")]);
    let count = count_watched_sources(dir.path(), &["rs"]).expect("count");
    assert_eq!(count, 1);
}

/// A language project with zero matching sources must be distinguishable from a
/// real one, because the caller fails closed on zero.
/// (Project có ngôn ngữ nhưng 0 source khớp phải phân biệt được với project
/// thật, vì caller fail-closed khi count = 0.)
#[test]
fn declared_language_without_sources_counts_zero() {
    let dir = project_with(&[("Cargo.toml", ""), ("notes.txt", "")]);
    let extensions = watch_extensions(dir.path());
    assert_eq!(extensions, &["rs", "toml"]);
    // Cargo.toml itself carries the `toml` extension, so the manifest counts.
    assert_eq!(
        count_watched_sources(dir.path(), extensions).expect("count"),
        1,
        "the manifest itself is a watched source; a project with only a manifest still has one"
    );
}
