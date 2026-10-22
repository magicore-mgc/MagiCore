// Wires the manifest_path owner; behavior lives in manifest_path.rs.
// Nối owner manifest_path; xử lý nằm trong manifest_path.rs.
mod error;
mod manifest_path;
pub(super) use self::error::ManifestPathError;
#[cfg(test)]
pub(super) use self::manifest_path::model_manifest_relative_path;
pub(super) use self::manifest_path::{
    existing_or_encoded_manifest_path, model_manifest_dir, model_manifest_path, store_root,
    validate_model_name,
};
