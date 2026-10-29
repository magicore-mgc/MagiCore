//! Layout tests: every path derives from the MagiCore data root.
//! Test layout: mọi đường dẫn dẫn xuất từ root dữ liệu MagiCore.

use super::super::layout::{
    ACTIVE_LINK, CELLAR_DIR, KegLayout, OPT_DIR, PREFIX_DIR, validate_version,
};
use std::path::PathBuf;

#[test]
fn every_layout_path_lives_under_the_data_root() {
    let layout = KegLayout::with_data_root("/sandbox/data");
    let version = validate_version("1.2.3").unwrap();
    let paths = [
        layout.lib_root(),
        layout.cellar_root(),
        layout.opt_root(),
        layout.prefix_root(),
        layout.keg_dir("libcxx", &version),
        layout.opt_dir("libcxx"),
        layout.library_prefix_dir("libcxx"),
        layout.version_prefix_dir("libcxx", &version),
        super::super::active::pointer_path(&layout, "libcxx"),
        super::super::keg::KegStore::new(layout.clone())
            .keg_dir("libcxx", "1.2.3")
            .unwrap(),
    ];
    for path in paths {
        assert!(
            path.starts_with("/sandbox/data"),
            "{} escaped the data root",
            path.display()
        );
    }
}

#[test]
fn layout_documents_the_homebrew_shape() {
    let layout = KegLayout::with_data_root("/d");
    let version = validate_version("0.1.0").unwrap();
    assert_eq!(
        layout.keg_dir("z", &version),
        PathBuf::from("/d/lib")
            .join(CELLAR_DIR)
            .join("z")
            .join("0.1.0")
    );
    assert_eq!(
        layout.opt_dir("z").join(ACTIVE_LINK),
        PathBuf::from("/d/lib")
            .join(OPT_DIR)
            .join("z")
            .join(ACTIVE_LINK)
    );
    assert_eq!(
        layout.version_prefix_dir("z", &version),
        PathBuf::from("/d/lib")
            .join(PREFIX_DIR)
            .join("z")
            .join("0.1.0")
    );
}

#[test]
fn resolved_layout_uses_the_mgc_platform_data_root() {
    // The resolution seam must stay mgc_platform-owned; this asserts the
    // relationship without mutating the process environment.
    // (Seam resolve thuộc mgc_platform; kiểm quan hệ, không sửa env tiến trình.)
    let layout = KegLayout::with_data_root(mgc_platform::paths::GlobalPaths::new().unwrap().root);
    assert!(layout.lib_root().ends_with("lib"));
    assert!(layout.lib_root().starts_with(&layout.data_root));
}
