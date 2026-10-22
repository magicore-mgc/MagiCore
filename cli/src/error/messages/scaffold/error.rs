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
