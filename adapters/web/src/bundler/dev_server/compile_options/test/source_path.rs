#![allow(clippy::unwrap_used)]
//! Nearest-owner compile-options regression tests.
//! Test regression gần owner compile-options.

use super::super::{COMPILER_IDENTITY, COMPILER_VERSION};
use super::support::canonical_compile_options_ok;
use tempfile::tempdir;

/// Same-content sources at DIFFERENT logical paths in one project get
/// different keys (sourcemap `sources` entries differ per path).
/// (Source cùng nội dung ở path logic KHÁC nhau trong 1 project có key
/// khác nhau (mục `sources` trong sourcemap khác theo path).)
#[test]
fn different_source_paths_get_distinct_keys() {
    let tmp = tempdir().unwrap();
    let project = tmp.path();
    std::fs::create_dir_all(project.join("src")).unwrap();
    std::fs::create_dir_all(project.join("lib")).unwrap();
    let bytes = b"export const same = true;";
    std::fs::write(project.join("src/a.ts"), bytes).unwrap();
    std::fs::write(project.join("lib/b.ts"), bytes).unwrap();

    let source_digest = blake3::Hasher::new()
        .update(bytes)
        .finalize()
        .to_hex()
        .to_string();
    let key_src = mgc_store::cas::CompilationKey::new(
        &source_digest,
        mgc_store::cas::Loader::Ts,
        COMPILER_IDENTITY,
        COMPILER_VERSION,
        &canonical_compile_options_ok(project, &project.join("src/a.ts")),
    )
    .unwrap();
    let key_lib = mgc_store::cas::CompilationKey::new(
        &source_digest,
        mgc_store::cas::Loader::Ts,
        COMPILER_IDENTITY,
        COMPILER_VERSION,
        &canonical_compile_options_ok(project, &project.join("lib/b.ts")),
    )
    .unwrap();
    assert_ne!(
        key_src, key_lib,
        "equal bytes at different logical paths must not share a cache entry"
    );
}
