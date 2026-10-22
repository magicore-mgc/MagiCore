//! Container-local typed error export boundary.
//! Ranh giới export lỗi typed cục bộ cho container.

#[path = "error.rs"]
mod error;

pub(super) use error::{
    container_manifest_required, container_profile_unsupported,
    web_container_runtime_split_unsupported,
};
