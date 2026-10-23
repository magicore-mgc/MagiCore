//! Safe tarball/cache file opening for extraction.
//! Mở file tarball/cache an toàn cho giải nén.

use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult};
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub(crate) fn open_tarball_path(pkg: &ResolvedPackage, path: &Path) -> MgResult<File> {
    mgc_platform::open_regular_file_no_follow(path).map_err(|err| {
        MgError::Other(format!(
            "failed to open tarball '{}' for '{}': {}",
            path.display(),
            pkg.id.name_str(),
            err
        ))
    })
}

pub(crate) fn open_rewound_cache_file(source: &Arc<Mutex<File>>) -> MgResult<File> {
    let mut file = source
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    file.seek(SeekFrom::Start(0))
        .map_err(|err| MgError::Other(format!("failed to rewind verified cache file: {err}")))?;
    file.try_clone()
        .map_err(|err| MgError::Other(format!("failed to clone verified cache file: {err}")))
}
