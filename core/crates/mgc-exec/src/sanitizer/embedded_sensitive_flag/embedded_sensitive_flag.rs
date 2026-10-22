use crate::sanitizer::is_sensitive_flag;

/// True when one argv item contains a shell-like command with a sensitive flag.
/// It recognizes shell continuations, quote concatenation and escaped characters.
pub(crate) fn contains_embedded_sensitive_flag(arg: &str) -> bool {
    // Shell removes backslash-newline pairs before parsing a token. Join both
    // common newline forms before splitting so `--to\\\nken` is seen as `--token`.
    let joined_lines = arg.replace("\\\r\n", "").replace("\\\n", "");
    joined_lines.split_whitespace().any(|token| {
        // Shell concatenates adjacent quoted/unquoted fragments and removes
        // backslash escapes before passing argv to the child.
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

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
