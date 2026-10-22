//! Trusted publishing, lock signatures, and trust policy diagnostics.
//! Lỗi trusted publishing, chữ ký lock và trust policy.

use anyhow::Error;
use anyhow::anyhow;

/// A v4 lock cannot be signed while a legacy detached signature is present.
/// Không ký lock v4 khi sidecar chữ ký legacy vẫn tồn tại.
pub fn lock_v4_legacy_signature_sidecar() -> Error {
    anyhow!(
        "v4 lockfile cannot use a legacy .lock.sig sidecar; remove or explicitly migrate the stale sidecar before signing"
    )
}

/// Refuse to re-sign a v4 document whose existing integrity evidence fails.
/// Từ chối ký lại v4 nếu bằng chứng toàn vẹn hiện hữu không hợp lệ.
pub fn lock_v4_existing_signature_invalid() -> Error {
    anyhow!("refusing to re-sign a v4 lockfile whose existing digest or signature is invalid")
}

pub fn trusted_registry_admin_required() -> Error {
    anyhow!("trusted publisher management requires --admin-token or MAGICORE_REGISTRY_ADMIN_TOKEN")
}

pub fn trusted_binding_failed(status: u16) -> Error {
    anyhow!("trusted publisher binding request failed with HTTP {status}")
}

pub fn trusted_registry_url_invalid() -> Error {
    anyhow!("trusted publishing requires a valid registry URL without credentials or fragment")
}

pub fn trusted_registry_requires_https() -> Error {
    anyhow!("trusted publishing requires HTTPS except for loopback registry URLs")
}

pub fn trusted_audience_required() -> Error {
    anyhow!("trusted publishing audience cannot be empty")
}

pub fn trusted_token_exchange_failed(status: u16) -> Error {
    anyhow!("trusted publishing token exchange failed with HTTP {status}")
}

pub fn trusted_oci_upload_start_failed(status: u16) -> Error {
    anyhow!("OCI blob upload start returned HTTP {status} instead of 202")
}

pub fn trusted_oci_upload_chunk_failed(status: u16) -> Error {
    anyhow!("OCI blob upload chunk returned HTTP {status} instead of 202")
}

pub fn trusted_oci_upload_complete_failed(status: u16) -> Error {
    anyhow!("OCI blob upload completion returned HTTP {status} instead of 201")
}

pub fn trusted_oci_upload_location_missing() -> Error {
    anyhow!("OCI registry omitted a required upload Location header")
}

pub fn trusted_oci_upload_range_invalid() -> Error {
    anyhow!("OCI registry returned an unexpected upload Range header")
}

pub fn trusted_token_response_invalid() -> Error {
    anyhow!("registry returned an invalid trusted publishing token response")
}

pub fn audit_signatures_flags_conflict() -> Error {
    anyhow!("audit signatures cannot be combined with vulnerability audit flags")
}

pub fn trusted_signatures_request_failed(status: u16) -> Error {
    anyhow!("trusted signatures request failed with HTTP {status}")
}

pub fn trusted_signatures_response_too_large() -> Error {
    anyhow!("trusted signatures response exceeds the configured size limit")
}

pub fn trusted_signatures_response_invalid() -> Error {
    anyhow!("registry returned an invalid trusted signatures response")
}

pub fn trusted_signatures_missing_or_mismatched() -> Error {
    anyhow!("registry returned no provenance entries or a package mismatch")
}

pub fn trusted_signatures_invalid() -> Error {
    anyhow!("trusted provenance signature or transparency chain verification failed")
}

/// `mgc trust pending` could not open the machine-local policy database.
/// `mgc trust pending` không mở được database policy cục bộ.
pub fn trust_policy_database_open_failed(path: &std::path::Path) -> Error {
    anyhow!(
        "failed to open trust policy database '{}': database access failed",
        path.display()
    )
}

/// `mgc trust pending` could not read the machine-local policy database.
/// `mgc trust pending` không đọc được policy từ database cục bộ.
pub fn trust_policy_database_read_failed(path: &std::path::Path) -> Error {
    anyhow!(
        "failed to read trust policies from '{}': database query failed",
        path.display()
    )
}

/// A trust-policy write or prune could not update its database.
/// Không thể ghi hoặc dọn database trust policy.
pub fn trust_policy_database_write_failed(
    path: &std::path::Path,
    reason: &dyn std::fmt::Display,
) -> Error {
    anyhow!(
        "failed to update trust policy database '{}': {reason}",
        path.display()
    )
}

/// A trust-policy mutation could not obtain the project writer lock.
/// Không lấy được khóa ghi project để sửa trust policy.
pub fn trust_policy_lock_failed(root: &std::path::Path, reason: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "cannot update trust policy in '{}' while project state is changing or unrecovered: {reason}",
        root.display()
    )
}

/// A package identifier is required for trust approval or denial.
/// Cần package identifier để approve hoặc deny trust.
pub fn trust_policy_package_empty() -> Error {
    anyhow!("package name cannot be empty")
}

/// `mgc trust anchor` found no core marker to attest.
/// (Không có core marker để chứng thực.)
pub fn trust_anchor_no_marker(root: &std::path::Path) -> Error {
    anyhow!(
        "no core marker in '{}' — run `mgc init` first, then `mgc trust anchor`",
        root.display()
    )
}

/// `mgc trust anchor` cannot access the home directory for key storage.
/// (Không truy cập được home để lưu key.)
pub fn trust_anchor_no_home(reason: &dyn std::fmt::Display) -> Error {
    anyhow!("cannot access global identities dir: {reason}")
}

/// `mgc trust anchor` failed to attest the project core.
/// (Chứng thực core thất bại.)
pub fn trust_anchor_failed(reason: &dyn std::fmt::Display) -> Error {
    anyhow!("core attestation failed: {reason}")
}

/// `--rotate` generates a fresh key by definition; naming a key with it
/// is contradictory — use plain `mgc trust anchor --key-id` to switch.
/// (`--rotate` sinh key mới theo định nghĩa; chỉ định key cùng lúc là
/// mâu thuẫn.)
pub fn trust_anchor_rotate_needs_generated_key() -> Error {
    anyhow!(
        "--rotate generates a fresh key and cannot be combined with --key-id (use --key-id alone to switch keys)"
    )
}

/// A trust-policy storage path is not a regular project-owned path.
/// Đường dẫn lưu trust policy không thuộc cây thư mục project an toàn.
pub fn trust_policy_path_invalid(path: &std::path::Path) -> Error {
    anyhow!(
        "refusing unsafe trust policy storage path '{}' (symlinks and non-regular paths are not allowed)",
        path.display()
    )
}
