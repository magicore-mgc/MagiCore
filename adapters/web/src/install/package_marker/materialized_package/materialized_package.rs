//! Check materialized package identity/content and persist its marker.
//! Kiểm tra identity/nội dung package đã materialize và lưu marker.

use std::path::Path;

use mgc_types::{MgResult, PackageId};

use crate::cache::ExtractedPackageMarker;
use crate::install::package_marker::{
    extracted_content_matches, read_extracted_package_marker, write_extracted_package_marker,
};
use crate::lockfile::installed_package_matches;

pub fn materialized_package_matches(
    target_root: &Path,
    package_id: &PackageId,
    source_marker: Option<&ExtractedPackageMarker>,
) -> MgResult<bool> {
    if !installed_package_matches(target_root, package_id) {
        return Ok(false);
    }

    let Some(source_marker) = source_marker else {
        return Ok(true);
    };

    let Some(target_marker) = read_extracted_package_marker(target_root)? else {
        return Ok(false);
    };

    Ok(target_marker == *source_marker && extracted_content_matches(target_root, source_marker)?)
}

pub fn write_materialized_package_marker(
    target_root: &Path,
    source_marker: Option<&ExtractedPackageMarker>,
) -> MgResult<()> {
    if let Some(marker) = source_marker {
        write_extracted_package_marker(target_root, marker)?;
    }
    Ok(())
}
