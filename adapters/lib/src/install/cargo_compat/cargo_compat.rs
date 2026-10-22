//! Stable public routes for MGC-managed Cargo compatibility.
//! Các route public ổn định cho tương thích Cargo do MGC quản lý.

use mgc_types::MgResult;
use std::path::{Path, PathBuf};

/// Validate both lock projections and return MGC's directory source for a Rust build.
/// Xác minh hai lock projection và trả directory source của MGC cho bước build Rust.
pub fn validate_build(project_root: &Path) -> MgResult<Option<PathBuf>> {
    validate_build_for_core(project_root, "lib")
}

/// Validate a Cargo build owned by the requested MagiCore core.
/// Xác minh Cargo build thuộc core MagiCore được yêu cầu.
pub fn validate_build_for_core(
    project_root: &Path,
    expected_core: &str,
) -> MgResult<Option<PathBuf>> {
    super::validate_build_for_core_impl(project_root, expected_core)
}
