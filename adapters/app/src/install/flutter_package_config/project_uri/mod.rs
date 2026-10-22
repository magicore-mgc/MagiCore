//! Wires safe Flutter project-root URI conversion.
//! Wiring chuyển đổi URI root Flutter an toàn.

mod error;
mod project_uri;

use mgc_types::{MgError, MgResult};
use std::path::Path;

use project_uri::flutter_project_root_uri_inner;

pub(in crate::install) fn flutter_project_root_uri(
    project_root: &Path,
    current_dir: Option<&Path>,
) -> MgResult<String> {
    flutter_project_root_uri_inner(project_root, current_dir).map_err(MgError::from)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
