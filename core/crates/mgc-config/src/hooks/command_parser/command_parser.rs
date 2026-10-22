//! Parse a hook command into argv without invoking a shell.
//! Parse command hook thành argv mà không gọi shell.

use anyhow::{Result, bail};

pub(in crate::hooks::owners) fn parse_hook_command(cmd: &str) -> Result<Vec<String>> {
    if cmd.trim().is_empty() {
        return Ok(Vec::new());
    }
    let mut args = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut chars = cmd.chars().peekable();
    while let Some(ch) = chars.next() {
        match (quote, ch) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), c) => current.push(c),
            (None, '\'' | '"') => quote = Some(ch),
            (None, ' ' | '\t' | '\n' | '\r') => {
                if !current.is_empty() {
                    args.push(std::mem::take(&mut current));
                }
            }
            (None, '&') if matches!(chars.peek(), Some('&')) => {
                bail!("hook command contains shell control operator '&&', which is not allowed")
            }
            (None, '|' | ';' | '<' | '>' | '`' | '$' | '(' | ')') => {
                bail!("hook command contains shell metacharacter '{ch}', which is not allowed")
            }
            (None, c) => current.push(c),
        }
    }
    if quote.is_some() {
        bail!("hook command has an unterminated quote");
    }
    if !current.is_empty() {
        args.push(current);
    }
    Ok(args)
}
