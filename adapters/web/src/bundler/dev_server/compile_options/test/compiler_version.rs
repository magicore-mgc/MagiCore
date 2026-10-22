#![allow(clippy::unwrap_used)]
//! Nearest-owner compile-options regression tests.
//! Test regression gần owner compile-options.

use super::super::COMPILER_VERSION;

/// Drift gate (P0-3, Tech Lead vòng-6/7): the hardcoded `COMPILER_VERSION`
/// in the dev server must equal the esbuild-rs version actually locked in
/// the workspace. A `cargo update` changing esbuild-rs without bumping the
/// const would poison the compiled cache (new compiler → old key → stale
/// entries served as hits). This test discovers the workspace root above
/// CARGO_MANIFEST_DIR and parses Cargo.lock directly.
///
/// Cổng chống trôi (P0-3): const `COMPILER_VERSION` hardcode trong dev
/// server phải bằng phiên bản esbuild-rs thật sự lock trong workspace.
/// `cargo update` đổi esbuild-rs mà quên nâng const sẽ độc cache đã biên
/// dịch (compiler mới → key cũ → entry stale được phục vụ như hit). Test
/// tìm workspace root từ CARGO_MANIFEST_DIR và parse trực tiếp Cargo.lock.
#[test]
fn compiler_version_matches_locked_esbuild_rs() {
    let manifest_dir = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"),
    );
    let repo_root = manifest_dir
        .ancestors()
        .find(|path| path.join("Cargo.toml").is_file() && path.join("Cargo.lock").is_file())
        .expect("workspace root with Cargo.toml and Cargo.lock must be found");
    let lock = std::fs::read_to_string(repo_root.join("Cargo.lock"))
        .expect("Cargo.lock must be readable from the workspace root");

    // Parse the `[[package]] name = "esbuild-rs"` stanza for its version —
    // plain text scan, no lockfile parser needed for ONE field.
    // (Quét stanza `[[package]] name = "esbuild-rs"` lấy version — text
    // scan, không cần parser lockfile cho MỘT field.)
    let locked = lock
        .split("name = \"esbuild-rs\"")
        .nth(1)
        .and_then(|rest| rest.split("version = \"").nth(1))
        .and_then(|rest| rest.split('"').next())
        .unwrap_or("<esbuild-rs not found in Cargo.lock>");
    assert_eq!(
        locked, COMPILER_VERSION,
        "esbuild-rs in Cargo.lock is {locked} but COMPILER_VERSION is {COMPILER_VERSION}; \
         update the compiler identity with the workspace pin to prevent stale cache entries"
    );
}
