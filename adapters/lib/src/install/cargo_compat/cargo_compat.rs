//! Stable public routes for MGC-managed Cargo compatibility.
//! Các route public ổn định cho tương thích Cargo do MGC quản lý.

use mgc_types::MgResult;
use std::path::{Path, PathBuf};

/// Validate both lock projections and return MGC's directory source for a Rust build.
///
/// Preflights the managed store because the legacy callers are offline build
/// lanes: without network there is no other source for a crate.
/// Xác minh hai lock projection và trả directory source của MGC cho bước build
/// Rust. Preflight store managed vì caller cũ là lane build offline — không mạng
/// thì không còn nguồn khác cho crate.
pub fn validate_build(project_root: &Path) -> MgResult<Option<PathBuf>> {
    validate_build_for_core(project_root, "lib")
}

/// Validate a Cargo build owned by the requested MagiCore core.
///
/// Offline semantics: every locked crate must already be verified in the store.
/// Ngữ nghĩa offline: mọi crate trong lock phải đã được xác minh trong store.
pub fn validate_build_for_core(
    project_root: &Path,
    expected_core: &str,
) -> MgResult<Option<PathBuf>> {
    validate_build_for_core_with_fetch(project_root, expected_core, true)
}

/// Validate a Cargo build for an explicit dependency-fetch mode.
///
/// `offline = false` skips the store preflight because Cargo can fetch; the
/// lock projection and core ownership checks still run.
/// Xác minh Cargo build theo chế độ lấy dependency tường minh. `offline = false`
/// bỏ preflight store vì Cargo tự tải được; vẫn kiểm lock projection và core.
pub fn validate_build_for_core_with_fetch(
    project_root: &Path,
    expected_core: &str,
    offline: bool,
) -> MgResult<Option<PathBuf>> {
    super::validate_build_for_core_impl(project_root, expected_core, offline)
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
