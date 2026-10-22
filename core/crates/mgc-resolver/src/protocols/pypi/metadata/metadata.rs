use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub(in crate::protocols::pypi) struct PypiJson {
    #[serde(default)]
    pub(in crate::protocols::pypi) info: PypiInfo,
    #[serde(default)]
    pub(in crate::protocols::pypi) releases: HashMap<String, Vec<PypiFile>>,
}

#[derive(Debug, Deserialize, Default)]
pub(in crate::protocols::pypi) struct PypiInfo {
    #[serde(default)]
    pub(in crate::protocols::pypi) requires_dist: Option<Vec<String>>,
    #[serde(default)]
    pub(in crate::protocols::pypi) requires_python: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(in crate::protocols::pypi) struct PypiFile {
    #[serde(default)]
    pub(in crate::protocols::pypi) filename: String,
    #[serde(default)]
    pub(in crate::protocols::pypi) url: String,
    #[serde(default)]
    pub(in crate::protocols::pypi) digests: PypiDigests,
    #[serde(default)]
    pub(in crate::protocols::pypi) packagetype: String,
    #[serde(default)]
    pub(in crate::protocols::pypi) requires_python: Option<String>,
    #[serde(default)]
    pub(in crate::protocols::pypi) upload_time_iso_8601: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(in crate::protocols::pypi) struct PypiDigests {
    #[serde(default)]
    pub(in crate::protocols::pypi) sha256: String,
}
