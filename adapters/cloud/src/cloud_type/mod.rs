//! Cloud type detection exports — export nhận diện kiểu Cloud.

mod cloud_type;
mod detection;
mod manifest;

pub use cloud_type::CloudType;
pub use detection::detect_type;
pub(crate) use manifest::manifest_is_cloud;
