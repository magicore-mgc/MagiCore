//! Backward-compatible parser re-export tests.
//! Test các re-export parser tương thích ngược.

#[test]
fn legacy_nested_module_path_exposes_both_parsers() {
    assert!(super::super::parse_swift_package_deps("").is_empty());
    assert!(
        super::super::parse_dump_package("{}")
            .expect("test operation must succeed")
            .is_empty()
    );
}
