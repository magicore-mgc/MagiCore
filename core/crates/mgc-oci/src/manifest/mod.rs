//! OCI model compatibility facade.
//! Facade tương thích cho các model OCI.

mod descriptor;
mod image_config;
pub mod media_types;
mod oci_manifest;

pub use descriptor::OciDescriptor;
pub use image_config::{OciConfig, OciHistory, OciImageConfig, OciRootFs};
pub use oci_manifest::OciManifest;
