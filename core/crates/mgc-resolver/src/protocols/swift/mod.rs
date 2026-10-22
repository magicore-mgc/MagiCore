//! Swift registry archive resolver. Git-source dependencies are parsed but fail closed.
//! Resolver archive Swift. Dependency nguồn Git được parse nhưng fail-closed.
//!
//! Registry archives require a published SHA-256 checksum; versions use the highest SwiftPM
//! requirement match. Package.swift dependencies are scanned without invoking Swift, and Git/local
//! dependencies remain explicitly unsupported. The App adapter owns installation and checkout layout.
//! Archive registry bắt buộc có SHA-256; chọn version cao nhất khớp requirement SwiftPM.
//! Dependency Package.swift được quét mà không chạy Swift; Git/local vẫn unsupported rõ ràng.
//! Adapter App quản lý cài đặt và checkout.

pub mod archive_dependencies;
pub mod git_source;
pub mod package_manifest;
pub mod registry;
pub mod requirement_edit;
pub mod resolved_pins;
pub mod types;
pub mod version_requirement;

pub use git_source::{git_transport_allowed, git_url_to_name, is_git_name};
pub use package_manifest::{parse_dump_package, parse_swift_package_deps};
pub use registry::SwiftRegistryProtocol;
pub use requirement_edit::{bump_swift_requirement, remove_swift_requirement};
pub use resolved_pins::parse_package_resolved;
pub use types::{SwiftDep, SwiftResolvedPin};
pub use version_requirement::swift_matches;
