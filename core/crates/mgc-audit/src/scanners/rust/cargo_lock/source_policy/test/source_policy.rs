use super::*;

#[test]
fn accepts_only_the_three_existing_crates_io_source_forms() {
    for source in [
        "registry+https://github.com/rust-lang/crates.io-index",
        "sparse+https://index.crates.io/",
        "registry+https://index.crates.io/",
    ] {
        assert!(is_crates_io_registry(source));
    }
    assert!(!is_crates_io_registry(
        "registry+https://private.example/index"
    ));
}

#[test]
fn skipped_reason_keeps_the_existing_package_and_source_detail() {
    assert_eq!(
        skipped_reason(
            "private-crate",
            "2.1.0",
            "registry+https://private.example/index"
        ),
        "Cargo.lock package 'private-crate@2.1.0' uses unsupported source 'registry+https://private.example/index'"
    );
}
