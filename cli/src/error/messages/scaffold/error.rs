//! Project scaffold validation diagnostics.
//! Lỗi kiểm tra scaffold project.

use anyhow::Error;
use anyhow::anyhow;

pub fn package_json_root_object() -> Error {
    anyhow!("package.json root must be an object")
}

pub fn dir_already_exists(target: &std::path::Path) -> Error {
    anyhow!("Directory '{}' already exists", target.display())
}

pub fn invalid_project_name(project_name: &str) -> Error {
    anyhow!(
        "Invalid project name '{project_name}': must be a single path segment without \
         separators, '..', or absolute paths (path traversal rejected)"
    )
}

pub fn invalid_project_name_retries_exhausted(retries: u32) -> Error {
    anyhow!(
        "Project name rejected after {retries} attempts; wizard aborted \
         without creating anything (no fallback name is invented)"
    )
}

pub fn unsupported_scaffold_framework(framework: &str) -> Error {
    anyhow!("Unsupported scaffold framework '{framework}'")
}

pub fn unsupported_monorepo_frontend(frontend: &str) -> Error {
    anyhow!("Scaffold for monorepo frontend framework '{frontend}' is not implemented yet")
}

/// A `create-<core>` request named a framework the registry does not accept.
///
/// The list is included because the honest failure has to answer "what can I
/// type instead?" — failing with only the bad name sends the user to `--help`
/// for a catalog that `--help` does not print.
/// (Yêu cầu `create-<core>` nêu framework registry không nhận. Kèm danh sách
/// vì lỗi trung thực phải trả lời "thay bằng gì?" — chỉ báo tên sai thì người
/// dùng phải mở `--help` cho catalog mà `--help` không in.)
pub fn unknown_framework_for_core(core: &str, framework: &str, supported: &[&str]) -> Error {
    let mut sorted = supported.to_vec();
    sorted.sort_unstable();
    anyhow!(
        "'{framework}' is not a framework of the '{core}' core. Supported '{core}' frameworks \
         ({}): {}. Nothing was created.",
        sorted.len(),
        sorted.join(", ")
    )
}
