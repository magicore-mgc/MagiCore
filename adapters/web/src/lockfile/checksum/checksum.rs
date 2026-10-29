//! `checksum.rs` — integrity mode, SHA-512 and missing-checksum warning.
//! chế độ integrity, SHA-512 và cảnh báo thiếu checksum

use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use base64::Engine;
use mgc_lockfile::Lockfile;
use sha2::{Digest, Sha512};

const TARBALL_HASH_BUFFER_BYTES: usize = 64 * 1024;

/// Values that switch a boolean flag off when they are the whole value.
///
/// A flag is never read by presence alone: `X=false` and `X=0` are how a shell,
/// a `.env` file or a CI template disables a feature, and reading them as "set"
/// silently enables what the operator just turned off.
/// (Giá trị tắt cờ khi là toàn bộ nội dung. Không bao giờ đọc cờ chỉ theo sự
/// tồn tại: `X=false`/`X=0` là cách shell, `.env` hay template CI tắt tính
/// năng; đọc chúng là "đã đặt" sẽ bật lại thứ người vận hành vừa tắt.)
const DISABLED_FLAG_VALUES: &[&str] = &["0", "false", "no", "off"];

/// Whether a boolean environment flag is enabled.
///
/// Unset or empty is off; `0`/`false`/`no`/`off` (any case) is off; anything
/// else is on, which matches how these variables are conventionally written
/// (`X=1`, `X=true`, or a bare `X=` for "yes").
/// (Cờ boolean: chưa đặt hoặc rỗng là tắt; `0/false/no/off` là tắt; còn lại là
/// bật — đúng quy ước `X=1`/`X=true`.)
fn flag_enabled(name: &str) -> bool {
    match std::env::var(name) {
        Ok(value) => {
            let value = value.trim();
            if value.is_empty() {
                return false;
            }
            !DISABLED_FLAG_VALUES.contains(&value.to_ascii_lowercase().as_str())
        }
        Err(_) => false,
    }
}

/// Strict integrity is requested by either spelling.
///
/// `MAGICORE_STRICT_INTEGRITY=false` used to still enforce strict mode, so a
/// developer who explicitly disabled it saw failures they could not explain;
/// the flag now reads its value.
/// (`MAGICORE_STRICT_INTEGRITY=false` trước đây vẫn ép strict; nay cờ đọc theo
/// giá trị.)
pub fn strict_integrity_enforced() -> bool {
    flag_enabled("MAGICORE_STRICT_INTEGRITY") || flag_enabled("MGC_STRICT_INTEGRITY")
}

/// Escape hatch for tarballs without SRI evidence.
/// Escape hatch cho tarball không có bằng chứng SRI.
///
/// Fail-closed by default: a package without an integrity field cannot be
/// distinguished from poisoned bytes, so accepting it requires an explicit
/// opt-in that always logs a warning at the decision point.
/// Mặc định fail-closed: package không có integrity không phân biệt được với
/// bytes bị đầu độc, nên chấp nhận cần opt-in rõ ràng và luôn ghi cảnh báo.
pub fn allow_unsigned_integrity() -> bool {
    flag_enabled("MAGICORE_ALLOW_UNSIGNED_INTEGRITY")
        || flag_enabled("MGC_ALLOW_UNSIGNED_INTEGRITY")
}

/// Whether the lockfile-checksum warning is suppressed.
///
/// Suppression is the dangerous direction, so it must be an explicit opt-in:
/// `MAGICORE_WEB_SKIP_LOCKFILE_CHECKSUM=false` previously suppressed the warning
/// anyway, hiding exactly the condition the operator asked to be told about.
/// (Tắt cảnh báo là hướng nguy hiểm nên phải là opt-in rõ ràng:
/// `...=false` trước đây vẫn tắt cảnh báo, che đúng điều người vận hành muốn
/// được báo.)
pub(super) fn skip_lockfile_checksum_warning() -> bool {
    flag_enabled("MAGICORE_WEB_SKIP_LOCKFILE_CHECKSUM")
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
    if !strict_integrity_enforced() || skip_lockfile_checksum_warning() {
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
