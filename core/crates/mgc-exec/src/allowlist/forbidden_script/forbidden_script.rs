use crate::allowlist::FORBIDDEN_TOOLS;

/// Find forbidden package-manager tools anywhere in a shell-ish script.
/// Tìm tool PM bị cấm trong toàn bộ script, không chỉ token đầu tiên.
pub fn find_forbidden_tool_in_script(script: &str) -> Option<&'static str> {
    // Scan identifier boundaries inside inline code and quoted command strings,
    // not only shell whitespace; ambiguous mentions fail closed intentionally.
    // Quét ranh giới identifier cả trong code/chuỗi; trường hợp mơ hồ chủ động chặn.
    script
        .split(|c: char| {
            c.is_whitespace()
                || matches!(
                    c,
                    ';' | '&' | '|' | '(' | ')' | '<' | '>' | '\'' | '"' | '`' | '='
                )
        })
        .filter_map(super::normalize_script_token::normalize_script_token)
        .find_map(|token| {
            FORBIDDEN_TOOLS
                .iter()
                .copied()
                .find(|forbidden| token == *forbidden)
        })
}
