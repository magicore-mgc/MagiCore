pub(crate) fn normalize_script_token(token: &str) -> Option<String> {
    let trimmed = token
        .trim()
        .trim_matches(|c| matches!(c, '"' | '\'' | '`'))
        .trim();
    if trimmed.is_empty() {
        return None;
    }
    let base = trimmed
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(trimmed)
        .to_ascii_lowercase();
    let base = base
        .strip_suffix(".cmd")
        .or_else(|| base.strip_suffix(".exe"))
        .or_else(|| base.strip_suffix(".bat"))
        .or_else(|| base.strip_suffix(".ps1"))
        .or_else(|| base.strip_suffix(".cjs"))
        .or_else(|| base.strip_suffix(".mjs"))
        .or_else(|| base.strip_suffix(".js"))
        .or_else(|| base.strip_suffix(".phar"))
        .unwrap_or(&base)
        .to_string();
    // Recognize common executable entrypoint filenames passed through a
    // language runtime, not just direct package-manager commands.
    // Nhận diện entrypoint phổ biến chạy qua runtime, không chỉ lệnh gọi trực tiếp.
    Some(match base.as_str() {
        "npm-cli" => "npm".to_string(),
        "pnpm-cli" => "pnpm".to_string(),
        "yarn-cli" => "yarn".to_string(),
        "bun-cli" => "bun".to_string(),
        other => other.to_string(),
    })
}
