//! SBOM format tests — Test định dạng SBOM

use super::super::*;

#[test]
fn keeps_the_existing_public_format_variants() {
    let selected = SbomFormat::Spdx;
    assert_eq!(selected, SbomFormat::Spdx);
    assert_eq!(SbomFormat::CycloneDx, SbomFormat::CycloneDx);
}
