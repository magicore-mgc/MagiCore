//! Wiring facade for wheel RECORD paths and site-tree enumeration.
//! Facade wiring cho path RECORD và duyệt cây site.

#[path = "record_paths.rs"]
mod implementation;

pub(in crate::protocols::pypi) use implementation::{
    collect_regular_files, validate_record_path, verified_site_file_path,
};
