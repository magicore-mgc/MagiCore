#[path = "output_path.rs"]
mod implementation;

pub(in crate::commands::build::implementation) use implementation::ensure_go_build_output_dir;
pub(super) use implementation::ensure_go_build_output_file;
