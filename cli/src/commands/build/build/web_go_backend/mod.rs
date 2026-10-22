#[path = "error.rs"]
mod error;
use error::WebGoBuildError;

#[path = "web_go_backend.rs"]
mod implementation;
#[path = "output_path/mod.rs"]
mod output_path;

pub(super) use implementation::GO_BACKEND_LANGUAGE;
pub(super) use implementation::build_web_go_backend;
#[cfg(all(test, feature = "web"))]
pub(super) use output_path::ensure_go_build_output_dir;
