//! Scaffold-local typed errors (M002 structural slice S05).
//! Lỗi cục bộ của module scaffold — message giữ nguyên tiếng Anh.

use anyhow::{Error, anyhow};
use std::path::Path;

/// Template registry returned invalid JSON metadata.
/// Registry template trả metadata JSON không hợp lệ.
pub(crate) fn template_metadata_invalid() -> Error {
    anyhow!("template registry returned invalid JSON metadata")
}

/// Template core/name identity is invalid (traversal, drive prefix, control chars).
/// Định danh core/name của template không hợp lệ (traversal, drive prefix, ký tự điều khiển).
pub(crate) fn template_layer_identity_invalid() -> Error {
    anyhow!(
        "template core/name must identify a supported core and a relative layer without traversal, drive prefixes, or control characters"
    )
}

/// Refuse an unsafe, linked, or non-directory template cache path.
/// Từ chối đường dẫn cache template không an toàn, symlink hoặc không phải thư mục.
pub(crate) fn template_cache_unsafe_path(path: &Path) -> Error {
    anyhow!(
        "refusing unsafe, linked, or non-directory template cache path '{}'",
        path.display()
    )
}

/// Template layer is missing its template.toml contract.
/// Layer template thiếu contract template.toml.
pub(crate) fn template_layer_missing_manifest(layer: &str) -> Error {
    anyhow!("Template layer '{layer}' missing template.toml")
}

/// package.json scripts must remain a map of string commands.
/// Bảng scripts trong package.json phải giữ cấu trúc map lệnh dạng chuỗi.
pub(crate) fn package_json_scripts_object() -> Error {
    anyhow!("package.json scripts must be an object of string commands")
}

/// Unsupported web framework for scaffolding.
/// Framework web không được hỗ trợ khi scaffold.
pub(crate) fn unsupported_web_framework(framework: &str) -> Error {
    anyhow!("Unsupported web framework '{framework}'")
}

/// Atomic scaffolding failure; no partial project was left behind.
/// Scaffold thất bại nguyên tử; không để lại project dở dang.
pub(crate) fn scaffold_staging_failed(target: &Path, cause: Error) -> Error {
    anyhow!(
        "Scaffolding '{}' failed atomically; no partial project was left \
         behind. Cause: {cause}",
        target.display()
    )
}

/// Online generator output exceeds MagiCore safety limits.
/// Output của generator online vượt giới hạn an toàn của MagiCore.
pub(crate) fn web_generator_output_too_large() -> Error {
    anyhow!("online framework generator output exceeds MagiCore safety limits")
}

/// Another create for this project name appears to be in progress.
/// Một tiến trình create khác cho tên project này dường như đang chạy.
pub(crate) fn create_claim_conflict(claim: &Path) -> Error {
    anyhow!(
        "Another create for this project name appears to be in progress \
         (claim file '{}' exists). If no create is running, remove the claim \
         file and retry.",
        claim.display()
    )
}

/// Unsupported scaffold core.
/// Core scaffold không được hỗ trợ.
/// Web template path does not exist.
/// Đường dẫn template web không tồn tại.
pub(crate) fn web_template_path_missing(dir: &str) -> Error {
    anyhow!("Web template path '{dir}' does not exist")
}

/// Unsupported web scaffold mode.
/// Mode scaffold web không được hỗ trợ.
pub(crate) fn unsupported_web_mode(mode: &str) -> Error {
    anyhow!("Unsupported web mode '{mode}'")
}

/// Web template layer does not exist.
/// Layer template web không tồn tại.
pub(crate) fn web_template_layer_missing(layer: &str) -> Error {
    anyhow!("Web template layer '{layer}' does not exist")
}

/// Unsupported fullstack framework combination for composited web scaffold.
/// Tổ hợp framework fullstack không được hỗ trợ cho scaffold web ghép.
pub(crate) fn unsupported_fullstack_framework(combined: &str) -> Error {
    anyhow!("Unsupported fullstack framework '{combined}' for composited web scaffold")
}

/// Web scaffold needs both a frontend and a backend framework.
/// Scaffold web cần cả framework frontend và backend.
pub(crate) fn web_scaffold_needs_fe_be() -> Error {
    anyhow!("Web scaffold needs a frontend and backend framework")
}

/// Unsupported monorepo backend.
/// Backend monorepo không được hỗ trợ.
pub(crate) fn unsupported_monorepo_backend(backend: &str) -> Error {
    anyhow!("Unsupported monorepo backend '{backend}'")
}

/// Scaffold not implemented for this label at this layer.
/// Scaffold chưa được triển khai cho nhãn này tại layer này.
pub(crate) fn scaffold_not_implemented(label: &str, layer: &str) -> Error {
    anyhow!(
        "Scaffold for {} is not implemented yet at '{}'",
        label,
        layer
    )
}

/// Duplicate template target within a layer.
/// Target template trùng lặp trong một layer.
pub(crate) fn duplicate_template_target(target: &str, layer: &str) -> Error {
    anyhow!("Duplicate template target '{target}' in '{layer}'")
}

/// Template source does not exist in the layer.
/// Source template không tồn tại trong layer.
pub(crate) fn template_source_missing(source: &str, layer: &str) -> Error {
    anyhow!("Template source '{source}' does not exist in '{layer}'")
}

/// Binary template source cannot declare template context.
/// Source template nhị phân không được khai báo template context.
pub(crate) fn binary_source_with_context(label: &str) -> Error {
    anyhow!("Binary template source '{label}' cannot declare template context")
}

/// Missing template manifest for a layer.
/// Thiếu manifest template cho layer.
pub(crate) fn template_manifest_missing(layer: &str) -> Error {
    anyhow!("Missing template manifest '{layer}'")
}

/// Invalid manifest source/target path — template path must not escape root.
/// Đường dẫn template không hợp lệ — không được thoát root.
pub(crate) fn template_materialization_path_invalid(path: &str) -> Error {
    anyhow!("Unsafe template materialization path: {:?}", path)
}

/// Bound manifest parsing allocation — memory limit before manifest parse.
/// Giới hạn bộ nhớ trước khi parse manifest.
pub(crate) fn template_manifest_too_large(limit: u64) -> Error {
    anyhow!("Template manifest exceeds the {} byte limit", limit)
}

pub fn scaffold_source_unavailable(core: &str, framework: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "No scaffold source for core '{core}' framework '{framework}': the registry layer is unavailable (check network/registry) and no existing project with this framework was found on this machine. Options: (1) restore network access so the registry layer can be fetched, or (2) point at an existing project with --from-local <path>. No project was created."
    )
}
