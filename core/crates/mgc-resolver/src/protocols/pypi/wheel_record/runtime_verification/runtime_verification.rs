//! Coordinate archive identity, RECORD verification and exact site membership.
//! Điều phối identity archive, xác minh RECORD và membership site chính xác.

use crate::protocols::pypi::{PypiProtocol, verify_wheel_identity};
use mgc_types::{MgError, MgResult};
use std::collections::BTreeSet;
use std::path::Path;

impl PypiProtocol {
    pub(in crate::protocols::pypi) fn verify_runtime_materialization_contents(
        wheel_bytes: &[u8],
        expected_name: &str,
        expected_version: &str,
        site: &Path,
    ) -> MgResult<()> {
        let site_metadata = std::fs::symlink_metadata(site)?;
        if !site_metadata.file_type().is_dir() {
            return Err(MgError::Integrity(
                "Python runtime site is not a real directory".to_string(),
            ));
        }

        let entries = crate::protocols::zip_reader::read_zip_entries(wheel_bytes)?;
        let archive = super::super::archive_index::index_wheel_entries(entries)?;
        let record_bytes = archive.record_bytes().ok_or_else(|| {
            MgError::Integrity("authenticated Python wheel has no dist-info/RECORD".to_string())
        })?;
        let record = std::str::from_utf8(record_bytes)
            .map_err(|_| MgError::Integrity("Python wheel RECORD is not UTF-8".to_string()))?;
        let metadata = archive.metadata_bytes().ok_or_else(|| {
            MgError::Integrity("authenticated Python wheel has no dist-info/METADATA".to_string())
        })?;
        let record_path = archive.record_path().ok_or_else(|| {
            MgError::Integrity("authenticated Python wheel has no dist-info/RECORD".to_string())
        })?;
        let metadata_path = archive.metadata_path().ok_or_else(|| {
            MgError::Integrity("authenticated Python wheel has no dist-info/METADATA".to_string())
        })?;
        let record_root = record_path.split_once('/').map(|pair| pair.0);
        let metadata_root = metadata_path.split_once('/').map(|pair| pair.0);
        if record_root != metadata_root {
            return Err(MgError::Integrity(
                "Python wheel METADATA and RECORD belong to different dist-info directories"
                    .to_string(),
            ));
        }
        verify_wheel_identity(metadata, expected_name, expected_version)?;

        let expected_files = super::super::record_verification::verify_record_members(
            site,
            &archive,
            record_path,
            record,
        )?;
        if expected_files.len() != archive.member_count()
            || archive
                .member_names()
                .any(|name| !expected_files.contains(name))
        {
            return Err(MgError::Integrity(
                "authenticated wheel members and RECORD entries do not match".to_string(),
            ));
        }

        let mut actual_files = BTreeSet::new();
        super::super::record_paths::collect_regular_files(site, site, &mut actual_files)?;
        if actual_files != expected_files {
            return Err(MgError::Integrity(
                "Python runtime site contains unrecorded or missing files".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
