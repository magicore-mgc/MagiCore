/// Lowercase basename without Windows executable suffix.
/// Basename lowercase, bỏ hậu tố executable Windows.
pub(crate) fn tool_basename(program: &str) -> String {
    let base = program
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(program)
        .to_ascii_lowercase();
    base.strip_suffix(".exe")
        .or_else(|| base.strip_suffix(".cmd"))
        .unwrap_or(&base)
        .to_string()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
