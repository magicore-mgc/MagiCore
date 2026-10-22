//! Project-config path-guard wiring.
//! Wiring bộ kiểm tra path cấu hình project.
mod project_config_metadata;
pub(in crate::project::project) use project_config_metadata::ensure_regular_project_config;
pub(in crate::project::project::file_access) use project_config_metadata::ensure_regular_project_config_typed;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
