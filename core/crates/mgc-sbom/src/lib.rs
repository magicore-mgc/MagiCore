//! MagiCore SBOM (Software Bill of Materials) Generator
//! Tạo SBOM (Danh sách vật liệu phần mềm) theo chuẩn CycloneDX

pub mod cyclonedx;
mod error;
mod format;
pub mod generator;
mod mapping;
mod options;
mod spdx;

pub use cyclonedx::{Bom, Component, ComponentType, Dependency};
pub use error::{SbomError, SbomResult};
pub use format::SbomFormat;
pub use generator::SbomGenerator;
pub use options::SbomOptions;
