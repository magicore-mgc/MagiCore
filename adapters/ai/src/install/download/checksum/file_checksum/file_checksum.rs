use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Kiểm tra checksum file theo prefix thuật toán: `sha256:`/`blake3:`/bare hex (=blake3).
/// (Verify file checksum by algorithm prefix: `sha256:`/`blake3:`; bare hex means blake3.)
pub fn verify_file_checksum(path: &Path, expected: &str) -> MgResult<()> {
    let expected = expected.trim();
    let (algo, want) = match expected.split_once(':') {
        Some(("sha256", rest)) => ("sha256", rest),
        Some(("blake3", rest)) => ("blake3", rest),
        Some((other, _)) => {
            return Err(MgError::Other(format!(
                "unsupported checksum algorithm '{other}' (expected 'sha256:' or 'blake3:')"
            )));
        }
        None => ("blake3", expected),
    };

    let computed = match algo {
        "sha256" => {
            use sha2::Digest;
            use std::io::Read;
            let mut file = std::fs::File::open(path)?;
            let mut hasher = sha2::Sha256::new();
            let mut buffer = [0_u8; 64 * 1024];
            loop {
                let count = file.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            let digest = hasher.finalize();
            hex::encode(digest)
        }
        _ => mgc_crypto::Blake3Hasher::hash_file(path)
            .map_err(|error| MgError::Other(format!("hash model artifact: {error}")))?
            .to_hex(),
    };

    if computed.eq_ignore_ascii_case(want) {
        Ok(())
    } else {
        Err(MgError::Other(format!(
            "checksum mismatch ({algo}): expected {want}, got {computed}"
        )))
    }
}
