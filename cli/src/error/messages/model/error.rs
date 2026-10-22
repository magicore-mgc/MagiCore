//! Model artifact, source, and quantization diagnostics.
//! Lỗi artifact, nguồn model và quantization.

use anyhow::Error;
use anyhow::anyhow;

/// HF request failed
pub fn hf_request_failed() -> Error {
    anyhow!("HF request failed")
}

/// pull manifest failed — registry running?
pub fn pull_manifest_failed() -> Error {
    anyhow!("pull manifest failed (is the registry running? `mgc registry serve`)")
}

/// parse manifest failed
pub fn parse_manifest_failed() -> Error {
    anyhow!("parse manifest failed")
}

/// file does not exist
pub fn file_not_found(path: &std::path::Path) -> Error {
    anyhow!("file does not exist: {}", path.display())
}

/// path does not exist
pub fn path_not_found(p: &std::path::Path) -> Error {
    anyhow!("path does not exist: {}", p.display())
}

/// Patch metadata contains a path outside the project-owned patch store.
pub fn patch_path_rejected(reason: &str) -> Error {
    anyhow!("patch verification refused the configured patch path: {reason}")
}

/// A package/version-range patch identity already exists in the project.
pub fn patch_duplicate_identity() -> Error {
    anyhow!(
        "a patch already exists for this package and version range; remove it before adding a replacement"
    )
}

/// directory has no files
pub fn dir_has_no_files(p: &std::path::Path) -> Error {
    anyhow!("directory has no files: {}", p.display())
}

/// push model failed — server running?
pub fn push_model_failed() -> Error {
    anyhow!("push model failed — is the server running? (`mgc registry serve`)")
}

/// catalog listing failed
pub fn catalog_failed() -> Error {
    anyhow!("catalog listing failed")
}

/// token not found (revoked?)
pub fn token_not_found(token: &str) -> Error {
    anyhow!("Token {} not found (was it revoked?)", token)
}

pub fn invalid_oci_source(oci: &str) -> Error {
    anyhow!("invalid oci source '{oci}' — use `oci://registry/repo:tag`")
}

pub fn invalid_model_name(name: &str) -> Error {
    anyhow!("model name must be a safe relative path: '{name}'")
}

pub fn unsafe_model_manifest_path() -> Error {
    anyhow!("model manifest path contains a symlink or non-directory component")
}

pub fn hf_revision_required() -> Error {
    anyhow!("hf:// pulls require --revision with an immutable 40-hex commit SHA")
}

pub fn invalid_hf_revision() -> Error {
    anyhow!("HF revision must be an immutable 40-hex commit SHA")
}

pub fn hf_sha256_required() -> Error {
    anyhow!("hf:// pulls require --sha256 with the trusted 64-hex artifact digest")
}

pub fn invalid_hf_sha256() -> Error {
    anyhow!("HF SHA-256 must contain exactly 64 hexadecimal characters")
}

pub fn invalid_hf_source() -> Error {
    anyhow!("invalid hf source; expected hf://<org>/<model>/<file>")
}

pub fn hf_size_overflow() -> Error {
    anyhow!("HF artifact size overflow")
}

pub fn hf_download_limit_exceeded(limit: u64) -> Error {
    anyhow!("HF artifact exceeded --max-bytes ({limit}); download was not imported into CAS")
}

pub fn hf_content_length_mismatch(expected: u64, actual: u64) -> Error {
    anyhow!("HF artifact Content-Length mismatch: expected {expected}, received {actual}")
}

pub fn hf_sha256_mismatch(expected: &str, actual: &str) -> Error {
    anyhow!("HF artifact SHA-256 mismatch: expected {expected}, got {actual}")
}

pub fn hf_artifact_changed_during_import() -> Error {
    anyhow!("HF artifact changed while being imported into CAS")
}

pub fn model_pull_flags_require_hf() -> Error {
    anyhow!("--revision, --sha256, and --max-bytes apply only to hf:// model sources")
}

pub fn unsupported_quantize_target(target: &str) -> Error {
    anyhow!("unsupported target: {target} (use q4_k_m or q8_0; awq needs a GPU toolchain)")
}

pub fn llama_cpp_missing() -> Error {
    anyhow!(
        "llama_cpp is unavailable in the MagiCore-managed environment; this model operation is unsupported until MagiCore can resolve and install the required native artifact"
    )
}

pub fn llama_quantize_failed(code: Option<i32>) -> Error {
    anyhow!("llama_cpp.quantize fail (exit {code:?})")
}
