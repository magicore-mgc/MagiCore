//! Converts the accepted v4 artifact wire shape into the shared ArtifactRef type.
//! File này chỉ nối và export owner artifact_ref.

mod artifact_ref;

pub(super) use artifact_ref::deserialize_v4_artifact;
