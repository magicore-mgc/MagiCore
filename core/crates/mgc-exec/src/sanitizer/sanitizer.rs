//! Coordinates argv redaction before audit serialization.

use super::{REDACTED, is_sensitive_flag, redact_equals_value, redact_shell_argument};

/// Redact args: `--token=VAL` / `--token VAL` / `-p VAL` → `[REDACTED]`.
/// Value không nhạy cảm giữ nguyên.
pub fn redact_args(args: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(args.len());
    let mut consume_next = false;
    for arg in args {
        if consume_next {
            out.push(REDACTED.to_string());
            consume_next = false;
            continue;
        }
        if let Some(redacted) = redact_shell_argument(arg) {
            out.push(redacted);
            continue;
        }
        if let Some(redacted) = redact_equals_value(arg) {
            out.push(redacted);
            continue;
        }
        if is_sensitive_flag(arg) {
            // `--token` đứng riêng — value nằm arg kế
            out.push(arg.clone());
            consume_next = true;
        } else {
            out.push(arg.clone());
        }
    }
    if consume_next {
        // flag nhạy cảm đứng cuối không có value — không thêm REDACTED thừa
        out.push(REDACTED.to_string());
    }
    out
}
