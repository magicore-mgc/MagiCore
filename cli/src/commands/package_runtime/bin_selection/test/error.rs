//! Bin selection error contract regressions.
//! Test hồi quy contract lỗi chọn bin.

use super::*;

#[test]
fn bin_selection_failures_keep_typed_variants_and_messages() {
    let cases = [
        (
            registry_package_invalid_bin(),
            "package runner: invalid executable name; use a simple declared bin name",
            0,
        ),
        (
            registry_package_undeclared_bin(),
            "package runner: selected executable is not declared by the requested package",
            1,
        ),
        (
            registry_package_ambiguous_bin(),
            "package runner: package declares multiple executables; select one with --bin NAME",
            2,
        ),
    ];
    for (error, message, variant) in cases {
        assert_eq!(error.to_string(), message);
        let typed = error
            .downcast_ref::<BinSelectionError>()
            .expect("bin selection factory retains typed error");
        assert!(matches!(
            (typed, variant),
            (BinSelectionError::InvalidBin, 0)
                | (BinSelectionError::UndeclaredBin, 1)
                | (BinSelectionError::AmbiguousBin, 2)
        ));
    }
}
