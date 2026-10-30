//! Standard paths (RULE §12: no hardcoded paths).
//! Đường dẫn chuẩn: store/cache/logs/locks/dlx/advisories/patches/quarantine/registry + lock signatures.

mod data_dir;
mod global_paths;
mod project_paths;
mod project_root;
pub(crate) mod shell;

pub use data_dir::{
    MGC_DATA_DIR_ENV, global_bin_dir, global_tools_dir, mgc_data_dir, project_data_dir,
};
pub use global_paths::GlobalPaths;
pub use project_paths::ProjectPaths;
pub use project_root::find_project_root;
pub use shell::{ensure_bin_on_path, ensure_global_bin_on_path};
