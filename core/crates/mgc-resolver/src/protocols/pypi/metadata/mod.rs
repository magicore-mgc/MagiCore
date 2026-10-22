mod metadata;
#[cfg(test)]
pub(in crate::protocols::pypi) use metadata::PypiDigests;
pub(in crate::protocols::pypi) use metadata::{PypiFile, PypiInfo, PypiJson};
