//! Target-selection tests for the bundle owner.
//! Test chọn target cho owner bundle.

use super::super::target_from_str;

#[test]
fn target_table_keeps_known_targets_and_es2020_fallback() {
    assert!(matches!(target_from_str("es5"), esbuild_rs::Target::ES5));
    assert!(matches!(
        target_from_str("es2021"),
        esbuild_rs::Target::ES2021
    ));
    assert!(matches!(
        target_from_str("esnext"),
        esbuild_rs::Target::ESNext
    ));
    assert!(matches!(
        target_from_str("unsupported"),
        esbuild_rs::Target::ES2020
    ));
}
