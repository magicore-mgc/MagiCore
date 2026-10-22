//! Marker identity construction and package-local root selection.
//! Dựng định danh marker và chọn root package cục bộ.

use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use mgc_store::Layout;
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult};

use crate::cache::{
    ExtractedPackageMarker, TarballContentSignature, shared_extracted_package_root,
};

use super::{
    compute_sha256_hex, compute_sha256_hex_from_path, compute_sha256_hex_from_reader,
    tarball_content_signature, tarball_content_signature_from_path,
    tarball_content_signature_from_reader,
};

pub fn local_extracted_package_root(layout: &Layout, pkg: &ResolvedPackage) -> PathBuf {
    shared_extracted_package_root(layout.root(), pkg)
}

pub fn expected_extracted_package_marker_from_bytes(
    pkg: &ResolvedPackage,
    tarball_bytes: &[u8],
) -> MgResult<ExtractedPackageMarker> {
    let mut marker = expected_extracted_package_marker_fast(pkg, tarball_bytes);
    apply_content_signature(&mut marker, tarball_content_signature(tarball_bytes)?);
    Ok(marker)
}

pub fn expected_extracted_package_marker_from_path(
    pkg: &ResolvedPackage,
    tarball_path: &Path,
) -> MgResult<ExtractedPackageMarker> {
    let mut marker = expected_extracted_package_marker_fast_from_path(pkg, tarball_path)?;
    apply_content_signature(
        &mut marker,
        tarball_content_signature_from_path(tarball_path)?,
    );
    Ok(marker)
}

pub fn expected_extracted_package_marker_from_reader<R: Read + Seek>(
    pkg: &ResolvedPackage,
    reader: &mut R,
) -> MgResult<ExtractedPackageMarker> {
    let mut marker = expected_extracted_package_marker_fast_from_reader(pkg, reader)?;
    reader
        .seek(SeekFrom::Start(0))
        .map_err(|err| MgError::Other(format!("failed to rewind tarball reader: {err}")))?;
    let signature = tarball_content_signature_from_reader(&mut *reader)?;
    reader
        .seek(SeekFrom::Start(0))
        .map_err(|err| MgError::Other(format!("failed to rewind tarball reader: {err}")))?;
    apply_content_signature(&mut marker, signature);
    Ok(marker)
}

/// Build identity-only cache metadata without decompressing the tarball.
/// Dựng metadata định danh cache mà không giải nén tarball.
pub fn expected_extracted_package_marker_fast_from_path(
    pkg: &ResolvedPackage,
    tarball_path: &Path,
) -> MgResult<ExtractedPackageMarker> {
    let tarball_fingerprint = if pkg.integrity.is_empty() {
        compute_sha256_hex_from_path(tarball_path)?
    } else {
        format!("integrity:{}", pkg.integrity)
    };
    Ok(fast_marker(pkg, tarball_fingerprint))
}

pub fn expected_extracted_package_marker_fast_from_reader<R: Read + Seek>(
    pkg: &ResolvedPackage,
    reader: &mut R,
) -> MgResult<ExtractedPackageMarker> {
    reader
        .seek(SeekFrom::Start(0))
        .map_err(|err| MgError::Other(format!("failed to rewind tarball reader: {err}")))?;
    let tarball_fingerprint = if pkg.integrity.is_empty() {
        compute_sha256_hex_from_reader(&mut *reader)?
    } else {
        format!("integrity:{}", pkg.integrity)
    };
    reader
        .seek(SeekFrom::Start(0))
        .map_err(|err| MgError::Other(format!("failed to rewind tarball reader: {err}")))?;
    Ok(fast_marker(pkg, tarball_fingerprint))
}

pub fn expected_extracted_package_marker_fast(
    pkg: &ResolvedPackage,
    tarball_bytes: &[u8],
) -> ExtractedPackageMarker {
    let tarball_fingerprint = if pkg.integrity.is_empty() {
        compute_sha256_hex(tarball_bytes)
    } else {
        format!("integrity:{}", pkg.integrity)
    };
    fast_marker(pkg, tarball_fingerprint)
}

fn fast_marker(pkg: &ResolvedPackage, tarball_fingerprint: String) -> ExtractedPackageMarker {
    ExtractedPackageMarker {
        schema_version: 3,
        name: pkg.id.name_str().to_string(),
        version: pkg.id.version().to_string(),
        integrity: (!pkg.integrity.is_empty()).then(|| pkg.integrity.clone()),
        tarball_sha256: tarball_fingerprint,
        file_count: 0,
        unpacked_size: 0,
        file_tree_sha256: String::new(),
    }
}

fn apply_content_signature(
    marker: &mut ExtractedPackageMarker,
    signature: TarballContentSignature,
) {
    marker.file_count = signature.file_count;
    marker.unpacked_size = signature.unpacked_size;
    marker.file_tree_sha256 = signature.file_tree_sha256;
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
