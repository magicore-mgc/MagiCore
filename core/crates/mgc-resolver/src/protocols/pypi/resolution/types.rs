//! Data carried between PyPI resolution owners.
//! Dữ liệu được chuyển giữa các owner resolve PyPI.

use crate::protocols::pypi::{PypiFile, PypiInfo};
use mgc_types::Version;

pub(super) struct ReleaseCandidate {
    pub(super) version: Version,
    pub(super) files: Vec<PypiFile>,
}

pub(super) struct SelectedRelease {
    pub(super) version: Version,
    pub(super) files: Vec<PypiFile>,
    pub(super) consumer: Option<(u64, u64)>,
    pub(super) version_info: Option<PypiInfo>,
}
