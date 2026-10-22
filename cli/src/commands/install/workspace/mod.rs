//! Workspace manifest parsing and confined project discovery.
//! Đọc cấu hình workspace và tìm project trong đường dẫn được giới hạn.

mod workspace;

#[cfg(test)]
pub(super) use workspace::collect_installable_projects;
pub(crate) use workspace::discover_workspace_projects;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
