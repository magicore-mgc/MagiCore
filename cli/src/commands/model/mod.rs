//! Model command facade — mgc model push/pull.
//! (Facade lệnh model — mgc model push/pull.)
//!
//! AI core (Q11): `mgc model pull hf://org/model/file` or
//! `oci://registry/repo:tag` → CAS store and local manifest.
//! AI core (Q11): kéo model HF/OCI vào CAS và manifest cục bộ.
//!
//! This module preserves the public command path and wires operation owners.
//! Module này giữ đường dẫn lệnh công khai và nối các owner thao tác.
mod cas_pull;
mod dispatch;
mod huggingface;
mod local_models;
mod manifest_io;
mod manifest_path;
mod manifest_types;
mod oci_pull;
mod quantize;
mod registry_catalog;
mod registry_client;
mod registry_pull;
mod registry_push;
mod types;

pub use self::dispatch::run;
pub use self::types::{ModelArgs, ModelCmd};

#[cfg(test)]
use self::cas_pull::{cas_import, cas_pull};
#[cfg(test)]
use self::huggingface::{next_hf_download_total, parse_hf_locator};
#[cfg(test)]
use self::local_models::remove_local;
#[cfg(test)]
use self::manifest_io::{read_manifests_in, save_manifest_in};
#[cfg(test)]
use self::manifest_path::{model_manifest_relative_path, validate_model_name};
#[cfg(test)]
use self::manifest_types::ModelManifest;
#[cfg(test)]
use self::quantize::quantize;

#[cfg(test)]
#[path = "../../test/model_test.rs"]
mod tests;
