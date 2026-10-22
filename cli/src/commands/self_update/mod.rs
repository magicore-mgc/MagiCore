//! Secure self-update command and responsibility owners.
//! Lệnh cập nhật an toàn và các owner theo từng trách nhiệm.

mod archive;
mod checksum;
mod download;
pub(crate) mod error;
mod install;
mod manifest;
mod platform;
mod provenance;
mod self_update;
mod types;
mod version;

pub use platform::{artifact_name, host_labels};
pub use provenance::{ProvenanceMode, resolve_provenance_mode, trust_roots};
pub use self_update::run;
pub use types::ReleaseManifest;
pub use version::resolve_version;

pub(super) use archive::stage_binary;
pub(super) use checksum::{sha256_file_streaming, verify_checksum};
pub(super) use download::{MAX_DOC_BYTES, download_capped, download_file, download_text};
#[cfg(test)]
pub(in crate::commands) use install::probe_reports_version;
pub(super) use install::swap_binary;
pub(super) use manifest::verify_manifest;
pub(super) use platform::detect_variant;
#[cfg(test)]
pub(in crate::commands) use provenance::merge_roots;
pub(super) use version::RELEASE_REPO;
#[cfg(test)]
pub(in crate::commands) use version::is_valid_version;

#[cfg(test)]
#[path = "../test/self_update.rs"]
mod tests;
