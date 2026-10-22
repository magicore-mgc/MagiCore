//! Python project ownership and compatibility helpers.
//! Helper tương thích và xác định core sở hữu project Python.

use mgc_types::MgResult;
use std::path::Path;

/// Normalize a Python distribution name according to PEP 503 for manifest ↔
/// lock coverage comparisons (not for filesystem paths).
/// Chuẩn hóa tên distribution Python theo PEP 503 khi đối chiếu manifest/lock.
// COMPAT facade (S09a/P03) — delegates to mgc_python_service::install.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: native_python_path_entries_from_store (manifest/lock coverage
//   check; body stays in lib).
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub(super) fn normalize_python_distribution_name(name: &str) -> String {
    mgc_python_service::install::normalize_python_distribution_name(name)
}

/// Find dependency metadata that the current native PEP 621 lane does not
/// own. Presence alone is sufficient to block an ambient-runtime fallback;
/// this deliberately does not parse or execute those files.
/// Tìm manifest Python chưa thuộc lane native; chỉ cần tồn tại là chặn fallback.
// COMPAT facade (S09a/P03) — delegates to mgc_python_service::install.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: native_python_path_entries_from_store (foreign-manifest guard;
//   body stays in lib).
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub(super) fn external_python_manifest_name(project_root: &Path) -> MgResult<Option<String>> {
    mgc_python_service::install::external_python_manifest_name(project_root)
}

/// Resolve the core that owns this Python runtime without guessing across a
/// mixed-core lockfile. The legacy default matches lock writing for projects
/// with neither marker nor config (`lib`).
/// Xác định core sở hữu runtime Python; mặc định cũ chỉ dùng khi không mơ hồ.
// COMPAT facade (S09a/P03) — delegates to mgc_python_service::install.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: cargo_compat.rs rust_root_ids (Rust lane — generic owner read),
//   write_canonical_lock_with_roots (generic lock writer), project_v4_lock_to_
//   lib_model, native_python_path_entries_from_store + installed_packages
//   (Python lane; bodies stay in lib), lib install tests.
// NOTE: the single Rust-lane caller is covered by this identical-signature
//   facade; no behavior change.
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub(crate) fn python_project_owner(project_root: &Path) -> MgResult<String> {
    mgc_python_service::install::python_project_owner(project_root)
}

/// Select Python lock entries owned by the active core. Legacy entries are
/// accepted only if the lock has no competing core owner.
/// Chỉ chọn entry Python thuộc core hiện tại; lock cũ mơ hồ bị chặn.
// COMPAT facade (S09a/P03) — delegates to mgc_python_service::install.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: native_python_path_entries_from_store + installed_packages
//   (bodies stay in lib), lib install tests.
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub(crate) fn select_python_packages_for_owner<'a>(
    packages: &'a [mgc_lockfile::Package],
    owner_core: &str,
) -> MgResult<Vec<&'a mgc_lockfile::Package>> {
    mgc_python_service::install::select_python_packages_for_owner(packages, owner_core)
}

/// Validate untrusted lock fields before they become a filesystem path.
/// Python project names are single-segment names and PEP 440 versions must
/// not contain path separators or platform-special characters.
/// (Validate field lock không tin cậy trước khi tạo path filesystem.)
// COMPAT facade (S09a/P03) — delegates to mgc_python_service::install.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: native_python_path_entries_from_store (site-dir validation; body
//   stays in lib), lib install tests.
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub(crate) fn python_site_dirname(name: &str, version: &str) -> MgResult<String> {
    mgc_python_service::install::python_site_dirname(name, version)
}
