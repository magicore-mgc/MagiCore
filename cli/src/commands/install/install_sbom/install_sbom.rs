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
        Ok(()) => {
            record_identity(&output);
            Ok(Some(output))
        }
        Err(error) => {
            eprintln!("WARN: SBOM not written for this install: {error}");
            Ok(None)
        }
    }
}

/// Write the install SBOM's identity beside it (SEC-14).
/// Ghi identity của SBOM install bên cạnh nó (SEC-14).
///
/// The document described the graph but not itself, so two SBOMs could not be
/// told apart and nobody could check that the bytes on disk were the ones the
/// install produced. The sidecar records the BLAKE3 digest, size and the
/// producing tool version; it is written only after a successful generation.
/// (Tài liệu mô tả graph nhưng không mô tả chính nó, nên hai SBOM không phân
/// biệt được và không ai kiểm được bytes trên đĩa đúng là bản install sinh ra.
/// Sidecar ghi digest BLAKE3, kích thước và version tool tạo; chỉ ghi sau khi
/// sinh thành công.)
fn record_identity(sbom: &Path) {
    let digest = match mgc_crypto::blake3_signer::Blake3Hasher::hash_file(sbom) {
        Ok(digest) => digest,
        Err(error) => {
            eprintln!("WARN: install SBOM digest not recorded: {error}");
            return;
        }
    };
    let size = std::fs::metadata(sbom).map(|meta| meta.len()).unwrap_or(0);
    let identity = serde_json::json!({
        "file": sbom.file_name().and_then(|name| name.to_str()).unwrap_or_default(),
        "blake3": digest.to_hex(),
        "bytes": size,
        "tool_version": env!("CARGO_PKG_VERSION"),
    });
    let mut sidecar = sbom.as_os_str().to_os_string();
    sidecar.push(".identity.json");
    if let Err(error) = std::fs::write(
        PathBuf::from(sidecar),
        serde_json::to_vec_pretty(&identity).unwrap_or_default(),
    ) {
        eprintln!("WARN: install SBOM identity not written: {error}");
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
