//! Stable public API and local-error compatibility mapping.
//! API công khai ổn định và ánh xạ lỗi cục bộ tương thích.

use std::path::Path;

use mgc_types::{MgResult, ResolvedGraph};

/// Normalize a Python distribution name according to PEP 503 for manifest ↔
/// lock coverage comparisons (not for filesystem paths).
/// Chuẩn hóa tên distribution Python theo PEP 503 khi đối chiếu manifest/lock.
pub fn normalize_python_distribution_name(name: &str) -> String {
    super::distribution_name::normalize_python_distribution_name(name)
}

/// Find dependency metadata that the current native PEP 621 lane does not
/// own. Presence alone blocks an ambient-runtime fallback.
/// Tìm manifest dependency ngoài lane PEP 621; chỉ cần tồn tại là chặn fallback.
pub fn external_python_manifest_name(project_root: &Path) -> MgResult<Option<String>> {
    super::external_manifest::external_python_manifest_name(project_root).map_err(Into::into)
}

/// Resolve the core that owns this Python runtime without guessing across a
/// mixed-core lockfile. Legacy projects with no marker/config default to `lib`.
/// Xác định core sở hữu runtime Python; project cũ không có identity mặc định `lib`.
pub fn python_project_owner(project_root: &Path) -> MgResult<String> {
    super::core_owner::python_project_owner(project_root).map_err(Into::into)
}

/// Select Python lock entries owned by the active core. Legacy entries are
/// accepted only if the lock has no competing core owner.
/// Chỉ chọn entry Python thuộc core hiện tại; lock cũ mơ hồ bị chặn.
pub fn select_python_packages_for_owner<'a>(
    packages: &'a [mgc_lockfile::Package],
    owner_core: &str,
) -> MgResult<Vec<&'a mgc_lockfile::Package>> {
    super::core_owner::select_python_packages_for_owner(packages, owner_core).map_err(Into::into)
}

/// Validate untrusted lock fields before using them in a Python site path.
/// Xác thực field lock không tin cậy trước khi dùng trong đường dẫn site Python.
pub fn python_site_dirname(name: &str, version: &str) -> MgResult<String> {
    super::runtime_compatibility::python_site_dirname(name, version).map_err(Into::into)
}

/// Read a digest-addressed wheel cache entry without following final links,
/// then verify SHA-256 before allowing it to satisfy an install.
/// Đọc cache wheel không đi theo link cuối và xác minh SHA-256 trước khi dùng.
pub fn read_cached_python_artifact(
    path: &Path,
    expected_sha256: &str,
) -> MgResult<Option<Vec<u8>>> {
    super::artifact_cache::read_cached_python_artifact(path, expected_sha256).map_err(Into::into)
}

/// Reject resolved artifacts this native Python runtime cannot import.
/// Từ chối artifact đã resolve mà runtime Python native không thể import.
pub fn validate_python_importable_graph(graph: &ResolvedGraph) -> MgResult<()> {
    super::runtime_compatibility::validate_python_importable_graph(graph).map_err(Into::into)
}

/// Canonicalize a core identity shared by Python project and lock owners.
/// Chuẩn hóa identity core dùng chung cho project Python và lock.
pub fn canonical_core_name(value: &str) -> String {
    super::core_owner::canonical_core_name(value)
}
