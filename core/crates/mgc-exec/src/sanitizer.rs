//! Secret sanitizer — che đối số nhạy cảm trước khi ghi audit log (00-index §5.4)
//! (A13 99-report: token/password/key → [REDACTED]; không bao giờ vào log)

/// Liệt kê hợp phần nhạy cảm trong tên flag (prefix, case-insensitive).
const SENSITIVE_PARTS: &[&str] = &[
    "token",
    "password",
    "pass",
    "_authtoken",
    "_password",
    "key",
    "secret",
    "apikey",
    "api_key",
    "auth",
];

const REDACTED: &str = "[REDACTED]";

/// Flag ngắn coi là nhạy cảm trong mọi ngữ cảnh (00 §5.4: `-p` → REDACTED).
const SENSITIVE_SHORT_FLAGS: &[&str] = &["-p"];

/// True nếu `arg` là flag nhạy cảm (`--api-key=...`, `--token ...`, `_authToken=...`).
/// Match theo segment: tách flag thành từng đoạn bằng `-`/`_` rồi so khớp từ khóa.
/// (vd `--api-key` → [api, key]; `_authToken` → [authtoken]; `--password` → [password])
/// False positive (thừa redact) chấp nhận — an toàn hơn lộ secret.
fn is_sensitive_flag(arg: &str) -> bool {
    let lower = arg.to_ascii_lowercase();
    let flag = lower.trim_start_matches('-');
    if flag.is_empty() {
        return false;
    }
    for part in SENSITIVE_PARTS {
        let needle = part.trim_start_matches('_');
        if flag
            .split(['-', '_'])
            .any(|seg| seg == needle || seg.starts_with(needle))
        {
            return true;
        }
    }
    SENSITIVE_SHORT_FLAGS.contains(&arg)
}

/// True when one argv item contains a shell-like command with a sensitive flag.
/// Audit records redact the whole item because safely rewriting shell syntax
/// would require parsing it and could leave quoted or escaped secrets behind.
fn contains_embedded_sensitive_flag(arg: &str) -> bool {
    // Shell removes backslash-newline pairs before parsing a token. Join both
    // common newline forms before splitting so `--to\\\nken` is seen as
    // `--token`, including commands authored with Windows line endings.
    // Shell bỏ cặp backslash-newline trước khi phân tích token. Nối hai dạng
    // newline phổ biến trước khi tách để nhận ra `--to\\\nken` thành `--token`.
    let joined_lines = arg.replace("\\\r\n", "").replace("\\\n", "");
    joined_lines.split_whitespace().any(|token| {
        // Shell concatenates adjacent quoted/unquoted fragments and removes
        // backslash escapes before passing argv to the child. Normalize those
        // syntax markers before checking the option name.
        let unquoted = token
            .chars()
            .filter(|ch| !matches!(ch, '\'' | '"' | '`' | '\\'))
            .collect::<String>();
        let token = unquoted.trim_matches(|ch: char| {
            matches!(
                ch,
                ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}' | '&' | '|' | '<' | '>' | ':'
            )
        });
        let flag = token
            .split_once('=')
            .map_or(token, |(candidate, _)| candidate);
        is_sensitive_flag(flag)
    })
}

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
        if arg.chars().any(char::is_whitespace) && contains_embedded_sensitive_flag(arg) {
            // Nested command strings (for example `sh -c "npm ... --token ..."`)
            // are one argv value; redact it in full to avoid leaking shell syntax variants.
            out.push(REDACTED.to_string());
            continue;
        }
        if let Some((flag, _)) = arg.split_once('=') {
            // `--token=VAL` — ẩn value, giữ flag
            if is_sensitive_flag(flag) {
                out.push(format!("{flag}={REDACTED}"));
            } else {
                out.push(arg.clone());
            }
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
