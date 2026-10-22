//! Checks the shared MagiCore manifest marker at a project root.
//! Kiểm tra marker manifest MagiCore dùng chung tại project root.

use std::path::Path;

pub(crate) fn manifest_is_any_mg(root: &Path) -> bool {
    root.join("mgc.toml").is_file()
}
