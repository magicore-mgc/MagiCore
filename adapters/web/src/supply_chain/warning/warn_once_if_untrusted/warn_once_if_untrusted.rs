//! Emit the process-local warning for the explicit untrusted bypass.
//! In cảnh báo theo process khi dùng bypass untrusted tường minh.

#[cfg(test)]
use std::cell::Cell;
#[cfg(test)]
use std::sync::OnceLock;

/// Warning text for the documented escape hatch. Every use is reported, so
/// a silence-once guard would hide the second bypass in the same process.
/// (Text cảnh báo cho escape hatch đã tài liệu hóa. Mỗi lần dùng đều được báo,
/// nên cổng "chỉ một lần" sẽ che lần bypass thứ hai trong cùng process.)
const UNTRUSTED_BYPASS_WARNING: &str = "WARN: [magicore] WARNING: MAGICORE_ALLOW_UNTRUSTED=1 — supply-chain guards\n   \
     (24h quarantine + no-downgrade) are BYPASSED for this process.";

/// Warn on EVERY guard run that uses the bypass (requirement: the escape
/// hatch must log each use, never silently repeat). The once-per-process
/// helper below stays test-only.
/// (Cảnh báo MỖI lần guard chạy dưới bypass — escape hatch phải log từng lần.)
pub(in crate::supply_chain) fn warn_each_time_if_untrusted(allow_untrusted: bool) -> bool {
    if !allow_untrusted {
        return false;
    }
    eprintln!("{UNTRUSTED_BYPASS_WARNING}");
    true
}

/// Legacy once-per-process reminder. No live caller uses it now —
/// `warn_each_time_if_untrusted` replaced it so every bypass is reported.
/// (Nhắc nhở một lần mỗi process. Không còn caller thật — đã thay bằng
/// cảnh báo mỗi lần để lần bypass nào cũng được báo.)
#[cfg(test)]
fn warn_once_if_untrusted_with(allow_untrusted: bool, warned: &OnceLock<()>) -> bool {
    if !allow_untrusted {
        return false;
    }

    let emitted = Cell::new(false);
    warned.get_or_init(|| {
        eprintln!("{UNTRUSTED_BYPASS_WARNING}");
        emitted.set(true);
    });
    emitted.get()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
