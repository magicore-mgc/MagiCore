//! Project patch directory route tests.
//! Test đường dẫn thư mục patch của dự án.

#![allow(clippy::unwrap_used)]

use std::path::Path;

#[test]
fn resolves_project_patch_directory() {
    let root = Path::new("sample-project");
    let paths = super::get_patches_dir(Some(root)).expect("test operation must succeed");
    assert_eq!(
        paths,
        mgc_platform::paths::ProjectPaths::from_root(root)
            .patches_dir()
            .to_path_buf()
    );
}
