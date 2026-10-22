//! Index ZIP members and identify the distribution's root RECORD/METADATA.
//! Index member ZIP và nhận diện RECORD/METADATA gốc của distribution.

use crate::protocols::zip_reader::ZipEntry;
use mgc_types::{MgError, MgResult};
use std::collections::BTreeMap;

use super::super::record_paths::validate_record_path;

#[derive(Debug)]
pub(in crate::protocols::pypi::wheel_record) struct ArchiveIndex {
    entries: Vec<ZipEntry>,
    member_indices: BTreeMap<String, usize>,
    record_index: Option<usize>,
    metadata_index: Option<usize>,
}

impl ArchiveIndex {
    pub(in crate::protocols::pypi::wheel_record) fn record_bytes(&self) -> Option<&[u8]> {
        self.record_index
            .map(|index| self.entries[index].data.as_slice())
    }

    pub(in crate::protocols::pypi::wheel_record) fn record_path(&self) -> Option<&str> {
        self.record_index
            .map(|index| self.entries[index].name.as_str())
    }

    pub(in crate::protocols::pypi::wheel_record) fn metadata_bytes(&self) -> Option<&[u8]> {
        self.metadata_index
            .map(|index| self.entries[index].data.as_slice())
    }

    pub(in crate::protocols::pypi::wheel_record) fn metadata_path(&self) -> Option<&str> {
        self.metadata_index
            .map(|index| self.entries[index].name.as_str())
    }

    pub(in crate::protocols::pypi::wheel_record) fn member_bytes(
        &self,
        name: &str,
    ) -> Option<&[u8]> {
        self.member_indices
            .get(name)
            .map(|index| self.entries[*index].data.as_slice())
    }

    pub(in crate::protocols::pypi::wheel_record) fn member_count(&self) -> usize {
        self.member_indices.len()
    }

    pub(in crate::protocols::pypi::wheel_record) fn member_names(
        &self,
    ) -> impl Iterator<Item = &str> {
        self.member_indices.keys().map(String::as_str)
    }
}

pub(in crate::protocols::pypi::wheel_record) fn index_wheel_entries(
    entries: Vec<ZipEntry>,
) -> MgResult<ArchiveIndex> {
    let mut member_indices = BTreeMap::new();
    let mut record_index = None;
    let mut metadata_index = None;

    for (index, entry) in entries.iter().enumerate() {
        validate_record_path(&entry.name)?;
        if member_indices.insert(entry.name.clone(), index).is_some() {
            return Err(MgError::Integrity(format!(
                "Python wheel contains duplicate member '{}'",
                entry.name
            )));
        }

        // Only root dist-info describes this distribution; nested metadata is payload.
        // Chỉ dist-info ở root mô tả distribution; metadata lồng là payload.
        let is_root_metadata = entry.name.split('/').count() == 2;
        if is_root_metadata
            && entry.name.ends_with(".dist-info/RECORD")
            && record_index.replace(index).is_some()
        {
            return Err(MgError::Integrity(
                "Python wheel contains multiple dist-info/RECORD files".to_string(),
            ));
        }
        if is_root_metadata
            && entry.name.ends_with(".dist-info/METADATA")
            && metadata_index.replace(index).is_some()
        {
            return Err(MgError::Integrity(
                "Python wheel contains multiple dist-info/METADATA files".to_string(),
            ));
        }
    }

    Ok(ArchiveIndex {
        entries,
        member_indices,
        record_index,
        metadata_index,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
