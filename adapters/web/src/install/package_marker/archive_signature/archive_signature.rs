//! Public tarball entry points and gzip/tar orchestration.
//! Entry point tarball public và điều phối gzip/tar.

use std::path::Path;

use mgc_types::{MgError, MgResult};

use crate::cache::TarballContentSignature;

use super::entry_scan::read_tarball_files;
use super::tree_signature::signature_from_files;

pub fn tarball_content_signature(tarball_bytes: &[u8]) -> MgResult<TarballContentSignature> {
    tarball_content_signature_from_reader(std::io::Cursor::new(tarball_bytes))
}

pub fn tarball_content_signature_from_path(
    tarball_path: &Path,
) -> MgResult<TarballContentSignature> {
    let file = std::fs::File::open(tarball_path).map_err(|err| {
        MgError::Other(format!(
            "failed to open tarball '{}' for content signature: {}",
            tarball_path.display(),
            err
        ))
    })?;
    tarball_content_signature_from_reader(file)
}

pub fn tarball_content_signature_from_reader<R: std::io::Read>(
    reader: R,
) -> MgResult<TarballContentSignature> {
    let decoder = flate2::read::GzDecoder::new(reader);
    let mut archive = tar::Archive::new(decoder);
    signature_from_files(read_tarball_files(&mut archive)?)
}
