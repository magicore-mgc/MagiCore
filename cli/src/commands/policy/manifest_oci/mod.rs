//! OCI policy manifest channel wiring.
//! Nối kênh OCI cho manifest policy.

mod manifest_oci;
mod oci_reference;

pub use manifest_oci::refresh_manifest_from_oci;
pub use oci_reference::parse_oci_reference;
