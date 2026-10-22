use crate::allowlist::is_react_native_subdir;

#[test]
fn detector_requires_a_mobile_directory_path() {
    assert!(!is_react_native_subdir(None));
}
