//! Model artifact, source, and quantization diagnostics.
//! Lỗi artifact, nguồn model và quantization.

mod error;

pub use error::{
    catalog_failed, dir_has_no_files, file_not_found, hf_artifact_changed_during_import,
    hf_content_length_mismatch, hf_download_limit_exceeded, hf_request_failed,
    hf_revision_required, hf_sha256_mismatch, hf_sha256_required, hf_size_overflow,
    invalid_hf_revision, invalid_hf_sha256, invalid_hf_source, invalid_model_name,
    invalid_oci_source, llama_cpp_missing, llama_quantize_failed, model_pull_flags_require_hf,
    parse_manifest_failed, patch_duplicate_identity, patch_path_rejected, path_not_found,
    pull_manifest_failed, push_model_failed, token_not_found, unsafe_model_manifest_path,
    unsupported_quantize_target,
};
