mod dist_tag;
mod git_checks;
mod lifecycle;
mod npm_publish;
mod npm_tarball_upload;
mod project_publish;
mod publish;
mod registry_selection;
mod trusted_oci_artifacts;
mod trusted_oci_blob_upload;
mod trusted_oci_image;
mod trusted_oci_manifest;
mod trusted_oci_repository;
mod trusted_publish;
mod trusted_pypi_publish;
mod trusted_session;
mod workspace;

pub use publish::{PublishArgs, run, stage};

#[cfg(test)]
pub(in crate::commands) use git_checks::{
    GitReadQuery, git_checks, git_exec_options, parse_git_behind_count,
};
#[cfg(test)]
pub(in crate::commands) use lifecycle::publish_lifecycle_decision;
#[cfg(test)]
pub(in crate::commands) use trusted_oci_artifacts::publish_trusted_oci_artifacts;
#[cfg(test)]
pub(in crate::commands) use trusted_oci_blob_upload::{
    OCI_UPLOAD_CHUNK_BYTES, safe_oci_upload_location,
};
#[cfg(test)]
pub(in crate::commands) use trusted_oci_manifest::trusted_oci_artifact_manifest;
#[cfg(test)]
pub(in crate::commands) use trusted_oci_repository::effective_trusted_oci_repository;
#[cfg(test)]
pub(in crate::commands) use trusted_session::{
    TrustedPublishSession, docker_push_timeout, oci_request_timeout, trusted_npm_request_timeout,
    trusted_token_deadline, trusted_token_expires_within,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
