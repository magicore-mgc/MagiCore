//! Signed binary manifest owner wiring.
//! Wiring owner manifest binary có chữ ký.

mod binary_manifest;

pub use binary_manifest::{
    BinaryEntry, BinaryManifest, MANIFEST_DOMAIN, load_manifest, manifest_message, verify_manifest,
};
