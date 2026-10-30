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

/// Re-verify a managed Cargo registry source against its verified archive.
///
/// Exposed so a caller that only wants to confirm (or finish) an already
/// materialized source — for example a build repairing Cargo's readiness
/// marker — reuses the install-time verification instead of trusting the tree.
/// Xác minh lại source registry Cargo do MGC quản lý với archive đã xác minh.
/// Mở route để caller chỉ cần xác nhận (hoặc hoàn tất) source đã materialize —
/// ví dụ build vá marker sẵn sàng của Cargo — dùng lại phép xác minh lúc install
/// thay vì tin cây trên đĩa.
pub fn verify_managed_source(
    package_root: &Path,
    expected_package_sha256: &str,
    archive_bytes: &[u8],
) -> MgResult<()> {
    super::verify_source_checksum(package_root, expected_package_sha256, archive_bytes)
}
