//! `checksum.rs` — integrity mode, SHA-512 and missing-checksum warning.
//! chế độ integrity, SHA-512 và cảnh báo thiếu checksum

use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use base64::Engine;
use mgc_lockfile::Lockfile;
use sha2::{Digest, Sha512};

const TARBALL_HASH_BUFFER_BYTES: usize = 64 * 1024;

pub fn strict_integrity_enforced() -> bool {
    std::env::var("MAGICORE_STRICT_INTEGRITY").is_ok()
        || std::env::var("MGC_STRICT_INTEGRITY").is_ok()
}
pub fn compute_sha512_b64(bytes: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(bytes);
    base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
}

pub fn compute_tarball_integrity(bytes: &[u8]) -> String {
    format!("sha512-{}", compute_sha512_b64(bytes))
}

pub fn compute_tarball_integrity_reader<R: Read>(mut reader: R) -> io::Result<String> {
    let mut hasher = Sha512::new();
    let mut buffer = [0u8; TARBALL_HASH_BUFFER_BYTES];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
    ))
}
pub fn maybe_warn_missing_lockfile_checksum(project_root: &Path, lockfile: &Lockfile) {
    if !strict_integrity_enforced() || std::env::var("MAGICORE_WEB_SKIP_LOCKFILE_CHECKSUM").is_ok()
    {
        return;
    }

    let has_locked_content = !lockfile.packages.is_empty();
    if !has_locked_content {
        return;
    }

    static WARNED: OnceLock<Mutex<std::collections::HashSet<PathBuf>>> = OnceLock::new();
    let warned = WARNED.get_or_init(|| Mutex::new(std::collections::HashSet::new()));
    let path = project_root.join("mgc.lock");
    let mut guard = match warned.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    if guard.insert(path) {
        eprintln!(
            "WARNING: Lockfile checksum file (mgc.lock.sha256) not found - cannot verify integrity"
        );
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
