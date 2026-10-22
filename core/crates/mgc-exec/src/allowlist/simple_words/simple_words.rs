use anyhow::{Result, bail};

pub(in crate::allowlist) fn split_simple_words(script: &str) -> Result<Vec<String>> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for ch in script.chars() {
        if escaped {
            current.push(ch);
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
            } else {
                current.push(ch);
            }
            continue;
        }

        match ch {
            '\'' | '"' => quote = Some(ch),
            ch if ch.is_whitespace() => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            other => current.push(other),
        }
    }

    if escaped {
        bail!("script ends with an unfinished escape");
    }
    if quote.is_some() {
        bail!("script contains an unterminated quote");
    }
    if !current.is_empty() {
        words.push(current);
    }
    Ok(words)
}
