//! Data-root resolution owner: MGC_DATA_DIR override, managed tool roots and per-project data.
//! Owner resolve data-root: override MGC_DATA_DIR, root tool do MGC quản lý và dữ liệu từng project.

mod data_dir;

pub use data_dir::{
    MGC_DATA_DIR_ENV, global_bin_dir, global_tools_dir, mgc_data_dir, project_data_dir,
};
