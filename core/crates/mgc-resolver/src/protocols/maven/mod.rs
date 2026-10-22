//! Maven Central native engine (Java/Maven repositories).
//! Engine native Maven Central (repository Java/Maven).
//!
//! The engine keeps its existing public module routes while assigning each
//! implementation responsibility to a small owner module. Shared failures
//! use `MgError`/`MgResult`; integrity and cache path checks stay fail-closed.
//! Giữ nguyên đường dẫn API hiện hữu, chia phần triển khai theo module chủ
//! sở hữu nhỏ; dùng lỗi chung và giữ xác minh integrity/cache fail-closed.

mod pom_coordinates;
mod pom_dependencies;
mod pom_properties;
mod pom_xml;
mod registry;
mod registry_protocol;
mod types;
mod version_requirement;

#[cfg(test)]
mod test;

pub use pom_coordinates::{parse_parent_coords, pom_project_coordinates, substitute_properties};
pub use pom_dependencies::collect_pom_dependencies;
pub use pom_properties::{collect_managed_versions, collect_pom_properties};
pub use pom_xml::element_blocks;
pub use registry::MavenProtocol;
pub use types::PomDependency;
