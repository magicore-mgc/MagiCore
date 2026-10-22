//! Coordinates source validation, destination preparation, and safe tree copying.
//! Điều phối xác thực source, chuẩn bị destination và copy cây an toàn.

use std::path::{Path, PathBuf};

use super::destination;
use super::error::CopyTreeError;
use super::source_validation;
use super::tree_copy;

pub(in crate::commands::store) fn prepare_destination_root(
    destination_path: &Path,
) -> Result<PathBuf, CopyTreeError> {
    destination::prepare_destination_root(destination_path)
        .map_err(|source| CopyTreeError::Destination(anyhow::Error::new(source)))
}

pub(in crate::commands::store) fn ensure_empty_destination_directory(
    destination_path: &Path,
) -> Result<(), CopyTreeError> {
    destination::ensure_empty_destination_directory(destination_path)
        .map_err(|source| CopyTreeError::Destination(anyhow::Error::new(source)))
}

pub(in crate::commands::store) fn prospective_destination_root(
    destination_path: &Path,
) -> Result<PathBuf, CopyTreeError> {
    destination::prospective_destination_root(destination_path)
        .map_err(|source| CopyTreeError::Destination(anyhow::Error::new(source)))
}

pub(in crate::commands::store) fn copy_dir(
    source: &Path,
    destination_path: &Path,
) -> anyhow::Result<()> {
    let validated = validate_present_tree(source)?;
    copy_validated_tree(&validated, destination_path)
}

pub(in crate::commands::store) fn validate_present_tree(
    source: &Path,
) -> anyhow::Result<source_validation::ValidatedSourceTree> {
    source_validation::validate_present_source_tree(source).map_err(anyhow::Error::new)
}

pub(in crate::commands::store) fn copy_validated_tree(
    source: &source_validation::ValidatedSourceTree,
    destination_path: &Path,
) -> anyhow::Result<()> {
    copy_tree(source, destination_path).map_err(anyhow::Error::new)
}

fn copy_tree(
    source: &source_validation::ValidatedSourceTree,
    destination_path: &Path,
) -> Result<(), CopyTreeError> {
    let source_root = source.root();

    let destination_candidate = destination::prospective_destination_root(destination_path)
        .map_err(|source| CopyTreeError::Destination(anyhow::Error::new(source)))?;
    if destination_candidate.starts_with(source_root)
        || source_root.starts_with(&destination_candidate)
    {
        return Err(CopyTreeError::SourceDestinationOverlap {
            source_path: source_root.to_path_buf(),
            destination_path: destination_candidate,
        });
    }
    let destination_parent = destination_path.parent().ok_or_else(|| {
        CopyTreeError::Destination(anyhow::anyhow!(
            "destination path has no parent: {}",
            destination_path.display()
        ))
    })?;
    let canonical_parent = destination::prepare_destination_parent(destination_parent)
        .map_err(|source| CopyTreeError::Destination(anyhow::Error::new(source)))?;
    let destination_name = destination_path.file_name().ok_or_else(|| {
        CopyTreeError::Destination(anyhow::anyhow!(
            "destination path has no filename: {}",
            destination_path.display()
        ))
    })?;
    let destination_candidate_after_parent = canonical_parent.join(destination_name);
    if destination_candidate_after_parent != destination_candidate {
        return Err(CopyTreeError::Destination(anyhow::anyhow!(
            "destination escaped its canonical parent: {}",
            destination_candidate_after_parent.display()
        )));
    }
    let destination_root = destination::prepare_destination_root(destination_path)
        .map_err(|source| CopyTreeError::Destination(anyhow::Error::new(source)))?;
    if destination_root != destination_candidate {
        return Err(CopyTreeError::DestinationRootChanged {
            expected: destination_candidate,
            actual: destination_root,
        });
    }
    ensure_empty_destination_directory(&destination_root)?;

    tree_copy::copy_directory(
        source_root,
        &destination_root,
        source_root,
        &destination_root,
    )
    .map_err(|source| CopyTreeError::TreeCopy(anyhow::Error::new(source)))?;
    tree_copy::copy_links(
        source_root,
        &destination_root,
        source_root,
        &destination_root,
    )
    .map_err(|source| CopyTreeError::TreeCopy(anyhow::Error::new(source)))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
