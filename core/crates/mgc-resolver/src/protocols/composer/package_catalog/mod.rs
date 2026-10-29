//! Package catalog owner exports (Packagist p2 metadata).
//! Export hẹp các thành phần thuộc catalog package (metadata p2 Packagist).

pub(in crate::protocols::composer) mod dependency_metadata;
mod package_catalog;
mod package_name;
mod types;

pub(in crate::protocols::composer) use package_catalog::{
    DIST_REFERENCE_MARKER_PREFIX, project_entry, select_entry,
};
pub(in crate::protocols::composer) use package_name::validate_composer_package_name;
pub(in crate::protocols::composer) use types::PackagistDocument;
