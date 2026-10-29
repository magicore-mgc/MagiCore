//! Previous signing-key selection / Chọn key ký trước đó.

use std::path::Path;

use crate::attestation::{Attestation, PreviousKey};

/// Previous signing key for rotation continuity, if the stored
/// attestation names a different key — `None` on first attest,
/// unparseable slots, or same-key rotation.
/// (Key ký cũ để rotation liên tục — `None` khi attest đầu, slot hỏng,
/// hoặc cùng key.)
pub(in crate::attestation) fn previous_key_for(
    path: &Path,
    current_key_id: &str,
) -> Option<PreviousKey> {
    let text = std::fs::read_to_string(path).ok()?;
    let old: Attestation = serde_json::from_str(&text).ok()?;
    // P-8.5: the rotated-out key keeps a bounded grace period instead of
    // living forever. Without an expiry a compromised old key would stay valid
    // for every future verification.
    // (P-8.5: key bị rotation có thời gian ân hạn hữu hạn thay vì tồn tại mãi.
    // Không có hạn, key cũ bị lộ vẫn hợp lệ cho mọi lần verify sau.)
    (old.key_id != current_key_id).then_some(PreviousKey {
        key_id: old.key_id,
        public_key: old.public_key,
        expires_at: Some(rotation_grace_deadline()),
    })
}

/// Grace deadline for a rotated-out key: 30 days from now (RFC 3339, UTC).
/// Hạn ân hạn cho key bị rotation: 30 ngày kể từ bây giờ (RFC 3339, UTC).
///
/// Thirty days is long enough for one release cycle and short enough that a
/// leaked previous key stops working.
/// (Ba mươi ngày đủ cho một chu kỳ release và đủ ngắn để key cũ bị lộ ngừng
/// hoạt động.)
fn rotation_grace_deadline() -> String {
    format_rfc3339_utc(std::time::SystemTime::now() + std::time::Duration::from_secs(30 * 86_400))
}

/// Current instant as `YYYY-MM-DDTHH:MM:SSZ`.
/// Thời điểm hiện tại dạng `YYYY-MM-DDTHH:MM:SSZ`.
pub fn now_rfc3339() -> String {
    format_rfc3339_utc(std::time::SystemTime::now())
}

/// Format a system time as `YYYY-MM-DDTHH:MM:SSZ`.
/// Định dạng system time thành `YYYY-MM-DDTHH:MM:SSZ`.
fn format_rfc3339_utc(time: std::time::SystemTime) -> String {
    let seconds = time
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let (days, remainder) = (seconds / 86_400, seconds % 86_400);
    let (year, month, day) = civil_from_days(days as i64);
    let (hour, minute, second) = (remainder / 3600, (remainder % 3600) / 60, remainder % 60);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Convert days from the Unix epoch to a civil date (Howard Hinnant's algorithm).
/// Đổi số ngày từ epoch Unix sang ngày lịch (thuật toán Howard Hinnant).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// Whether a previous key's grace period has passed.
/// Key cũ đã hết thời gian ân hạn chưa.
///
/// An unparseable expiry is treated as expired: a signature with an unreadable
/// deadline must not be trusted.
/// (Hạn đọc không được coi là hết hạn: chữ ký có hạn không đọc được không được
/// tin.)
pub fn rotation_key_expired(expires_at: &str, now: &str) -> bool {
    match expires_at.cmp(now) {
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Equal => false,
        std::cmp::Ordering::Greater => false,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
