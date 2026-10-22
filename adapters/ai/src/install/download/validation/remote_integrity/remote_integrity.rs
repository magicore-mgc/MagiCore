use super::super::super::ModelSource;
use super::super::validate_checksum_spec;
use mgc_types::{MgError, MgResult};

pub(in crate::install) fn validate_remote_integrity(source: &ModelSource) -> MgResult<()> {
    if matches!(source, ModelSource::Local(_)) {
        return Ok(());
    }

    let checksum = source.checksum().ok_or_else(|| {
        MgError::Integrity("remote model sources require a trusted sha256/blake3 checksum".into())
    })?;
    validate_checksum_spec(checksum)?;

    if let ModelSource::HuggingFace { revision, .. } = source {
        let revision = revision.as_deref().ok_or_else(|| {
            MgError::Integrity(
                "Hugging Face model sources require an immutable 40-hex commit revision".into(),
            )
        })?;
        if revision.len() != 40 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(MgError::Integrity(
                "Hugging Face revision must be an immutable 40-hex commit SHA".into(),
            ));
        }
    }
    Ok(())
}
