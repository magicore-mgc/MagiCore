//! Maven registry responsibility owners and stable type export.
//! Wiring các owner registry Maven và export kiểu ổn định.
mod artifact_materialization;
mod coordinate_validation;
mod parent_version_resolution;
mod registry;
mod repository_transport;

pub use registry::MavenProtocol;
