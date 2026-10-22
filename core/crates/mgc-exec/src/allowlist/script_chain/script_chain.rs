use crate::allowlist::{ScriptInvocation, parse_script_invocation};
use anyhow::Result;

/// Parse top-level AND chains without interpreting quoted or escaped operators.
/// Parse chuỗi AND ngoài quote; giữ nguyên toán tử được quote hoặc escape.
pub fn parse_script_chain(script: &str) -> Result<Vec<ScriptInvocation>> {
    let mut commands = Vec::new();
    let mut quote = None;
    let mut escaped = false;
    let mut start = 0;
    let mut chars = script.char_indices().peekable();
    while let Some((index, ch)) = chars.next() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active) = quote {
            if ch == active {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '&' if chars.peek().is_some_and(|(_, next)| *next == '&') => {
                commands.push(parse_script_invocation(&script[start..index])?);
                chars.next();
                start = index + 2;
            }
            _ => {}
        }
    }
    commands.push(parse_script_invocation(&script[start..])?);
    Ok(commands)
}
