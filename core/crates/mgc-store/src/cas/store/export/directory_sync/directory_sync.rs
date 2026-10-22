//! Directory Sync owner.
//! Phụ trách directory sync trong luồng export CAS.

use std::path::PathBuf;

use crate::cas::store::StoreError;
use crate::cas::write::sync_parent_dir;

pub(in crate::cas::store) fn sync_parent_directories(
    directories: std::collections::BTreeSet<PathBuf>,
) -> Result<(), StoreError> {
    let mut first_error = None;
    for directory in directories {
        if let Err(error) = sync_parent_dir(&directory)
            && first_error.is_none()
        {
            first_error = Some(error);
        }
    }
    first_error.map_or(Ok(()), Err)
}
