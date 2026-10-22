//! SBOM options default tests — Test mặc định tùy chọn SBOM

use super::super::*;

#[test]
fn preserves_the_existing_default_values() {
    let options = SbomOptions::default();

    assert!(!options.include_dev);
    assert!(options.include_licenses);
    assert!(options.include_hashes);
    assert_eq!(options.format, crate::SbomFormat::CycloneDx);
}
