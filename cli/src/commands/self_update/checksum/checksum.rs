//! Hash archives incrementally and verify the SHA-256 sidecar.
//! Hash archive theo khối và xác minh file SHA-256 đi kèm.

use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;

/// Hash a file with SHA-256 in 64 KiB streaming reads — the 512 MiB
/// archive is NEVER fully buffered in RAM (P1: the old read-all-then-hash
/// doubled peak memory on top of the download staging).
/// (Hash file dạng stream — archive không bao giờ nằm trọn trong RAM.)
pub(in crate::commands) fn sha256_file_streaming(path: &std::path::Path) -> Result<(u64, String)> {
    use sha2::Digest;
    use std::io::Read;
    let mut file =
        std::fs::File::open(path).map_err(|e| anyhow::anyhow!("read {}: {e}", path.display()))?;
    let mut hasher = sha2::Sha256::new();
    let mut total: u64 = 0;
    let mut chunk = [0u8; 65536];
    loop {
        let n = file
            .read(&mut chunk)
            .map_err(|e| anyhow::anyhow!("read {}: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        total += n as u64;
        hasher.update(&chunk[..n]);
    }
    Ok((total, hex::encode(hasher.finalize())))
}

/// Verify `sha256  filename` checksum text against the file. Mismatch =
/// delete nothing (caller temp dir cleans itself) + hard error BEFORE
/// any extract or swap.
/// (Verify checksum — lệch là fail cứng trước mọi bước sau.)
pub(in crate::commands) fn verify_checksum(
    archive_path: &std::path::Path,
    checksum_text: &str,
) -> Result<()> {
    let expected = checksum_text
        .split_whitespace()
        .next()
        .ok_or_else(|| {
            SelfUpdateError::download_failed(
                &archive_path.display().to_string(),
                "checksum file is empty",
            )
        })?
        .to_lowercase();
    let (_, actual) = sha256_file_streaming(archive_path)?;
    if actual != expected {
        return Err(SelfUpdateError::checksum_mismatch(
            &archive_path.display().to_string(),
            &expected,
            &actual,
        ));
    }
    Ok(())
}
