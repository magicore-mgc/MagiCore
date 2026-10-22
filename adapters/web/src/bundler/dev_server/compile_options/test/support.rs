#![allow(clippy::unwrap_used)]
//! Nearest-owner compile-options regression tests.
//! Test regression gần owner compile-options.

/// Helper: canonical options that must build — panics with the tsconfig
/// failure otherwise (fail-closed contract is tested separately).
/// (Helper: options chuẩn phải dựng được — panic kèm lỗi tsconfig (hợp
/// đồng fail-closed được test riêng).)
pub(super) fn canonical_compile_options_ok(
    root: &std::path::Path,
    source: &std::path::Path,
) -> String {
    super::super::canonical_compile_options(root, source).expect("canonical options must build")
}
