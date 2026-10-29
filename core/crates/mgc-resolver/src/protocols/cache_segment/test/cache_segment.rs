//! SEC-N1 cache path segment tests: traversal, separators, Windows quirks.
//! Test segment path cache SEC-N1: traversal, dấu phân cách, quirks Windows.

use super::super::validate_cache_segment;

#[test]
fn ordinary_names_are_accepted() {
    for name in ["six", "demo.lib", "1.0.0", "pkg-1.0.0", "@scope"] {
        assert_eq!(validate_cache_segment(name, "test").unwrap(), name);
    }
}

#[test]
fn traversal_and_separators_are_refused() {
    for name in [
        "..",
        ".",
        "",
        "   ",
        "../outside",
        "a/b",
        r"a\b",
        "a\0b",
        "/abs",
    ] {
        let error = validate_cache_segment(name, "test")
            .err()
            .map(|error| error.to_string());
        assert!(error.is_some(), "'{name}' must be refused");
    }
}

#[test]
fn windows_quirks_are_refused() {
    // `C:evil` is drive-relative on Windows and `name:$DATA` is an ADS.
    // (`C:evil` là tương đối ổ đĩa trên Windows và `name:$DATA` là ADS.)
    for name in [
        "C:evil",
        "name:$DATA",
        "trailing.",
    ] {
        let error = validate_cache_segment(name, "test")
            .err()
            .map(|error| error.to_string());
        assert!(error.is_some(), "{name:?} must be refused");
    }
}

#[test]
fn error_names_the_label_and_value_without_echoing_paths() {
    let error = validate_cache_segment("../etc", "NuGet package id").unwrap_err();
    let message = error.to_string();
    assert!(message.contains("NuGet package id"), "{message}");
    assert!(message.contains("../etc"), "{message}");
}
