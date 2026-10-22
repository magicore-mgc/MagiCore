//! Package-root location and extraction API.
//! API tìm và bung package vào root.
mod package_root;
pub use package_root::{
    ensure_extracted_package_root, ensure_extracted_package_root_from_bytes,
    ensure_extracted_package_root_from_file, locate_package_dir,
};
