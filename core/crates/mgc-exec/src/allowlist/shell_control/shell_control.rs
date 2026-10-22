use anyhow::{Result, bail};

/// Reject shell control characters that would require `sh -c` semantics.
/// Chặn chaining/redirection/subshell để beta chạy fail-closed.
pub fn reject_shell_control(script: &str) -> Result<()> {
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for ch in script.chars() {
        if escaped {
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            }
            continue;
        }

        match ch {
            '\'' | '"' => quote = Some(ch),
            ch if ch.is_control()
                || matches!(ch, ';' | '&' | '|' | '<' | '>' | '(' | ')' | '$' | '`') =>
            {
                let printable = match ch {
                    '\n' => "\\n".to_string(),
                    '\r' => "\\r".to_string(),
                    '\t' => "\\t".to_string(),
                    other => other.to_string(),
                };
                bail!(
                    "script uses unsupported shell control token '{printable}'; MagiCore runs scripts without a shell in beta"
                );
            }
            _ => {}
        }
    }
    Ok(())
}
