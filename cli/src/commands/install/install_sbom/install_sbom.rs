//! Write a supply-chain SBOM after every successful install.
//! Ghi SBOM chuỗi cung ứng sau mỗi lần install thành công.
//!
//! `mgc sbom` already generates CycloneDX/SPDX from the lockfile, but nothing
//! wrote one unless the user asked. An install that silently produces no
//! machine-readable inventory cannot be audited, so the artifact is emitted
//! next to `mgc.lock` on success.
//! `mgc sbom` đã sinh được CycloneDX/SPDX từ lockfile, nhưng chỉ khi user yêu
//! cầu. Một lần install không sinh inventory máy đọc được thì không audit
//! được, nên artifact được ghi cạnh `mgc.lock` khi thành công.

use std::path::{Path, PathBuf};

use anyhow::Result;

/// Package metadata the SBOM document names.
/// Metadata package mà tài liệu SBOM nêu tên.
struct Component {
    name: String,
    version: Option<String>,
}

/// Discover the project component name/version the same way `mgc sbom` does.
/// Dò tên/phiên bản component giống `mgc sbom`.
fn component(root: &Path) -> Component {
    let name = root
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());
    let version = std::fs::read_to_string(root.join("package.json"))
        .ok()
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        .and_then(|manifest| {
            manifest
                .get("version")
                .and_then(|value| value.as_str())
                .map(String::from)
        });
    Component { name, version }
}

/// Emit `sbom.cyclonedx.json` beside the lockfile, replacing a previous one.
///
/// Returns `Ok(None)` when no lockfile exists yet (nothing to inventory): an
/// install that wrote no lockfile is not a reason to fail the whole command.
/// Ghi `sbom.cyclonedx.json` cạnh lockfile, thay bản trước. Trả `Ok(None)` khi
/// chưa có lockfile (không có gì để kiểm kê): install không ghi lockfile không
/// phải lý do làm hỏng cả lệnh.
pub(crate) async fn write_sbom(root: &Path) -> Result<Option<PathBuf>> {
    if !root.join("mgc.lock").is_file() {
        return Ok(None);
    }
    let output = root.join("sbom.cyclonedx.json");
    // A previous SBOM describes a previous graph; remove it before generating
    // so a failure cannot leave a stale inventory looking current.
    // SBOM cũ mô tả graph cũ; xoá trước khi sinh để lỗi không để lại
    // inventory cũ trông như mới.
    let _ = std::fs::remove_file(&output);

    let component = component(root);
    let generated = crate::commands::sbom::run(
        Some("cyclonedx-json".to_string()),
        Some(output.clone()),
        Some(component.name),
        component.version,
        Some(root.to_path_buf()),
    )
    .await;

    // SBOM generation is evidence, not a gate: a lockfile the project policy
    // rejects (for example `require` signature policy on a fresh unsigned
    // lock) is already enforced by the install path. Turning that into an
    // install failure would report the same policy twice and break the
    // documented empty-install contract.
    // Sinh SBOM là bằng chứng, không phải cổng chặn: lockfile bị policy từ chối
    // (ví dụ policy ký `require` trên lock mới chưa ký) đã được đường install
    // thi hành. Biến nó thành lỗi install sẽ báo trùng policy và phá hợp đồng
    // install rỗng đã tài liệu hoá.
    match generated {
        Ok(()) => Ok(Some(output)),
        Err(error) => {
            eprintln!("WARN: SBOM not written for this install: {error}");
            Ok(None)
        }
    }
}
