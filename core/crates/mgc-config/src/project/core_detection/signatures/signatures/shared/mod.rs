//! Shared manifest recognizer wiring.
//! Wiring nhận diện manifest shared.
mod shared;
pub(in crate::project::project::core_detection) use shared::signature_file_exists;
pub(in crate::project::project::core_detection::signatures::signatures) use shared::{
    files_with_extension, has_named_dependency, has_regular_extension_file,
    signature_file_exists_in_project, text_contains_any,
};
