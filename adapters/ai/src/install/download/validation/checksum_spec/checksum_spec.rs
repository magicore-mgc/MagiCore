use mgc_types::{MgError, MgResult};

pub(in crate::install::download) fn validate_checksum_spec(expected: &str) -> MgResult<()> {
    let expected = expected.trim();
    let (algorithm, digest) = match expected.split_once(':') {
        Some(("sha256", digest)) => ("sha256", digest),
        Some(("blake3", digest)) => ("blake3", digest),
        Some((other, _)) => {
            return Err(MgError::Integrity(format!(
                "unsupported checksum algorithm '{other}' (expected 'sha256:' or 'blake3:')"
            )));
        }
        None => ("blake3", expected),
    };
    if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(MgError::Integrity(format!(
            "invalid {algorithm} checksum: expected exactly 64 hexadecimal characters"
        )));
    }
    Ok(())
}
